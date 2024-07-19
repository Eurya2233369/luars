use std::rc::Rc;

use super::chunk::{self, LocVar, Prototype, PrototypeBuilder, Upvalue};

#[derive(Debug)]
pub struct Reader {
    data: Vec<u8>,
}

impl Reader {
    pub fn new(data: Vec<u8>) -> Self {
        Reader { data }
    }

    pub fn check_header(&mut self) {
        assert_eq!(
            self.read_bytes(4),
            chunk::LUA_SIGNATURE,
            "not a precompiled chunk!"
        );
        assert_eq!(self.read_byte(), chunk::LUAC_VERSION, "version mismatch!");
        assert_eq!(self.read_byte(), chunk::LUAC_FORMAT, "format mismatch!");
        assert_eq!(self.read_bytes(6), chunk::LUAC_DATA, "corrupted!");
        assert_eq!(self.read_byte(), chunk::INT_SIZE, "int size mismatch!");
        assert_eq!(self.read_byte(), chunk::SIZET_SIZE, "size_t size mismatch!");
        assert_eq!(
            self.read_byte(),
            chunk::INSTRUCTION_SIZE,
            "instruction size mismatch!"
        );
        assert_eq!(
            self.read_byte(),
            chunk::LUA_INTEGER_SIZE,
            "lua_Integer size mismatch!"
        );
        assert_eq!(
            self.read_byte(),
            chunk::LUA_NUMBER_SIZE,
            "lua_Number size mismatch!"
        );
        assert_eq!(
            self.read_lua_integer(),
            chunk::LUAC_INT,
            "endianness mismatch!"
        );
        assert_eq!(
            self.read_lua_number(),
            chunk::LUAC_NUM,
            "float format mismatch!"
        );
    }

    pub fn read_byte(&mut self) -> u8 {
        self.data.remove(0)
    }

    fn read_u32(&mut self) -> u32 {
        let result = u32::from_le_bytes(self.data[..4].try_into().unwrap());
        self.data.drain(..4);
        result
    }

    fn read_u64(&mut self) -> u64 {
        let result = u64::from_le_bytes(self.data[..8].try_into().unwrap());
        self.data.drain(..8);
        result
    }

    fn read_lua_integer(&mut self) -> i64 {
        self.read_u64() as i64
    }

    fn read_lua_number(&mut self) -> f64 {
        let bit = self.read_u64();
        f64::from_bits(bit)
    }

    fn read_string(&mut self) -> String {
        let mut size = self.read_byte() as usize;
        if size == 0 {
            return String::new();
        }

        if size == 0xFF {
            size = self.read_u64() as usize;
        }

        let bytes = self.read_bytes(size - 1);
        String::from_utf8_lossy(&bytes).to_string()
    }

    fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let bytes = self.data[..n].into();
        self.data.drain(..n);
        bytes
    }

    pub fn read_proto(&mut self, parent_source: &str) -> Rc<Prototype> {
        let mut source: String = self.read_string();
        if source.is_empty() {
            source = parent_source.to_string();
        }
        Rc::new(
            PrototypeBuilder::new()
                .with_source(source.clone())
                .with_line_defined(self.read_u32())
                .with_last_line_defined(self.read_u32())
                .with_num_params(self.read_byte())
                .with_is_vararg(self.read_byte())
                .with_max_stack_size(self.read_byte())
                .with_code(self.read_func(|r| r.read_u32()))
                .with_constants(self.read_func(|r| r.read_constant()))
                .with_upvalues(self.read_func(|r| r.read_upvalue()))
                .with_protos(self.read_func(|r| r.read_proto(source.as_str())))
                .with_line_info(self.read_func(|r| r.read_u32()))
                .with_locvars(self.read_func(|r| r.read_locvar()))
                .with_upvalue_names(self.read_func(|r| r.read_string()))
                .build(),
        )
    }

    fn read_func<T, F>(&mut self, func: F) -> Vec<T>
    where
        F: Fn(&mut Reader) -> T,
    {
        let size: usize = self.read_u32().try_into().unwrap();
        let mut vec = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(func(self));
        }
        vec
    }

    fn read_constant(&mut self) -> chunk::ConstantType {
        let b = self.read_byte();
        match b {
            chunk::TAG_NIL => chunk::ConstantType::Nil,
            chunk::TAG_BOOLEAN => chunk::ConstantType::Boolean(self.read_byte() != 0),
            chunk::TAG_INTEGER => chunk::ConstantType::Integer(self.read_lua_integer()),
            chunk::TAG_NUMBER => chunk::ConstantType::Number(self.read_lua_number()),
            chunk::TAG_SHORT_STR | chunk::TAG_LONG_STR => {
                chunk::ConstantType::String(self.read_string())
            }
            _ => panic!("corrupted!"),
        }
    }

    fn read_upvalue(&mut self) -> Upvalue {
        Upvalue {
            instack: self.read_byte(),
            idx: self.read_byte(),
        }
    }

    fn read_locvar(&mut self) -> LocVar {
        LocVar {
            var_name: self.read_string(),
            start_pc: self.read_u32(),
            end_pc: self.read_u32(),
        }
    }
}
