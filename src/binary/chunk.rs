use std::rc::Rc;

pub const LUA_SIGNATURE: &[u8; 4] = b"\x1bLua";
pub const LUAC_VERSION: u8 = 0x53;
pub const LUAC_FORMAT: u8 = 0;
pub const LUAC_DATA: &[u8; 6] = b"\x19\x93\r\n\x1a\n";
pub const INT_SIZE: u8 = 4;
pub const SIZET_SIZE: u8 = 8;
pub const INSTRUCTION_SIZE: u8 = 4;
pub const LUA_INTEGER_SIZE: u8 = 8;
pub const LUA_NUMBER_SIZE: u8 = 8;
pub const LUAC_INT: i64 = 0x5678;
pub const LUAC_NUM: f64 = 370.5;

pub const TAG_NIL: u8 = 0x00;
pub const TAG_BOOLEAN: u8 = 0x01;
pub const TAG_NUMBER: u8 = 0x03;
pub const TAG_INTEGER: u8 = 0x13;
pub const TAG_SHORT_STR: u8 = 0x04;
pub const TAG_LONG_STR: u8 = 0x14;

#[derive(Debug)]
pub enum ConstantType {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
}

#[derive(Debug)]
struct Header {
    signature: [u8; 4],
    version: u8,
    format: u8,
    luac_data: [u8; 6],
    int_size: u8,
    sizet_size: u8,
    instruction_size: u8,
    lua_integer_size: u8,
    lua_number_size: u8,
    luac_int: i64,
    luac_num: f64,
}

#[derive(Debug)]
pub struct Prototype {
    source: String,
    line_defined: u32,
    last_line_defined: u32,
    num_params: u8,
    is_vararg: u8,
    max_stack_size: u8,
    code: Vec<u32>,
    constants: Vec<ConstantType>,
    upvalues: Vec<Upvalue>,
    protos: Vec<Rc<Prototype>>,
    line_info: Vec<u32>,
    locvars: Vec<LocVar>,
    upvalue_names: Vec<String>,
}

#[derive(Debug)]
pub struct Upvalue {
    pub instack: u8,
    pub idx: u8,
}

#[derive(Debug)]
pub struct LocVar {
    pub var_name: String,
    pub start_pc: u32,
    pub end_pc: u32,
}

impl Prototype {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            line_defined: 0,
            last_line_defined: 0,
            num_params: 0,
            is_vararg: 0,
            max_stack_size: 0,
            code: vec![],
            constants: vec![],
            upvalues: vec![],
            protos: vec![],
            line_info: vec![],
            locvars: vec![],
            upvalue_names: vec![],
        }
    }

    pub fn set_source(mut self, source: String) -> Self {
        self.source = source;
        self
    }

    pub fn set_line_defined(mut self, line_defined: u32) -> Self {
        self.line_defined = line_defined;
        self
    }

    pub fn set_last_line_defined(mut self, last_line_defined: u32) -> Self {
        self.last_line_defined = last_line_defined;
        self
    }

    pub fn set_num_params(mut self, num_params: u8) -> Self {
        self.num_params = num_params;
        self
    }

    pub fn set_is_vararg(mut self, is_vararg: u8) -> Self {
        self.is_vararg = is_vararg;
        self
    }

    pub fn set_max_stack_size(mut self, max_stack_size: u8) -> Self {
        self.max_stack_size = max_stack_size;
        self
    }

    pub fn set_code(mut self, code: Vec<u32>) -> Self {
        self.code = code;
        self
    }

    pub fn set_constants(mut self, constants: Vec<ConstantType>) -> Self {
        self.constants = constants;
        self
    }

    pub fn set_upvalues(mut self, upvalues: Vec<Upvalue>) -> Self {
        self.upvalues = upvalues;
        self
    }

    pub fn set_protos(mut self, protos: Vec<Rc<Prototype>>) -> Self {
        self.protos = protos;
        self
    }
    pub fn set_line_info(mut self, line_info: Vec<u32>) -> Self {
        self.line_info = line_info;
        self
    }

    pub fn set_locvars(mut self, loc_vars: Vec<LocVar>) -> Self {
        self.locvars = loc_vars;
        self
    }

    pub fn set_upvalue_names(mut self, upvalue_names: Vec<String>) -> Self {
        self.upvalue_names = upvalue_names;
        self
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn line_defined(&self) -> u32 {
        self.line_defined
    }

    pub fn last_line_defined(&self) -> u32 {
        self.last_line_defined
    }

    pub fn num_params(&self) -> u8 {
        self.num_params
    }

    pub fn is_vararg(&self) -> u8 {
        self.is_vararg
    }

    pub fn max_stack_size(&self) -> u8 {
        self.max_stack_size
    }

    pub fn code(&self) -> &Vec<u32> {
        &self.code
    }

    pub fn constants(&self) -> &Vec<ConstantType> {
        &self.constants
    }

    pub fn upvalues(&self) -> &Vec<Upvalue> {
        &self.upvalues
    }

    pub fn protos(&self) -> &Vec<Rc<Prototype>> {
        &self.protos
    }

    pub fn line_info(&self) -> &Vec<u32> {
        &self.line_info
    }

    pub fn locvars(&self) -> &Vec<LocVar> {
        &self.locvars
    }

    pub fn upvalue_names(&self) -> &Vec<String> {
        &self.upvalue_names
    }
}
