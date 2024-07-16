mod binary;
mod vm;
mod api;
mod state;
mod number;

use std::{
    fs::File, io::{BufReader, Read}, path::Path
};

use api::{basic::{Arithmetic, BasicType, Comparison}, lua_vm::LuaAPI };
use binary::{chunk::Prototype, un_dump};
use state::LuaState;

fn main() {
    /*
    let path = "/home/eurya/project/luars/test/luac.out";

    match read_file(path) {
        Ok(mut content) => {
            let proto = un_dump(&mut content);
            list(proto);
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }*/

    /*let mut test = state::new_lua_state();
    test.push_integer(1);
    test.push_number(2.0);
    test.push_string("3.0".to_string());
    test.push_string("4.0".to_string());
    print_stack(&test);

    test.arith(Arithmetic::LUA_OPADD);
    print_stack(&test);*/
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = Vec::new();
    reader.read_to_end(&mut contents)?;
    Ok(contents)
}

fn list(f: Prototype) {
    print_header(&f);
}

fn print_header(f: &Prototype) {
    let mut func = "main";
    if f.line_defined() > 0 {
        func = "function";
    }

    let mut vararg_flag = "";
    if f.is_vararg() > 0 {
        vararg_flag = "+";
    }

    println!(
        "{func} <{}: {} {}>",
        f.source(),
        f.last_line_defined(),
        f.code().len()
    );
    println!(
        "{}{vararg_flag} params, {} slots, {} upvalues",
        f.num_params(),
        f.max_stack_size(),
        f.upvalues().len()
    );
    println!(
        "{} locals, {} constants, {} functions",
        f.locvars().len(),
        f.constants().len(),
        f.protos().len()
    );
}

fn print_stack(ls: &LuaState) {
    let top = ls.top();
    println!("stack top {top}");
    for i in 1..=top {
        let t = ls.type_enum_id(i);
        match t {
            BasicType::LUA_TBOOLEAN => println!("[{:?}, {}]", t, ls.to_boolean(i)),
            BasicType::LUA_TNUMBER => println!("[{:?}, {}]", t, ls.to_number(i)),
            BasicType::LUA_TSTRING => println!("[{:?}, {}]", t, ls.to_string(i)),
            _ => println!("[{}]", ls.type_name_str(t)),
        }
    }
}
