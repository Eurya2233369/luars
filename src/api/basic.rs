#[macro_export]
macro_rules! basic {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl $name {
            pub fn index(&self) -> i8 {
                self.clone() as i8
            }
        }

        impl std::convert::TryFrom<i8> for $name {
            type Error = ();

            fn try_from(v: i8) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i8 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

basic! {
    #[derive(Debug, Clone, PartialEq)]
    pub enum BasicType {
        LUA_TNONE = -1,
        LUA_TNIL,
        LUA_TBOOLEAN,
        LUA_TLIGHTUSERDATA,
        LUA_TNUMBER,
        LUA_TSTRING,
        LUA_TTABLE,
        LUA_TFUNCTION,
        LUA_TUSERDATA,
        LUA_TTHREAD,
    }
}

basic! {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Arithmetic {
        LUA_OPADD,  // +
        LUA_OPSUB,  // -
        LUA_OPMUL,  // *
        LUA_OPMOD,  // %
        LUA_OPPOW,  // ^
        LUA_OPDIV,  // /
        LUA_OPIDIV, // //
        LUA_OPBAND, // &
        LUA_OPBOR,  // |
        LUA_OPBXOR, // ~
        LUA_OPSHL,  // <<
        LUA_OPSHR,  // >>
        LUA_OPUNM,  // - (unaryminus)
        LUA_OPBNOT, // ~
    }
}

basic! {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Comparison {
        LUA_OPEQ, // ==
        LUA_OPLT, // <
        LUA_OPLE, // <=
    }
}
