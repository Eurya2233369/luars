mod api;
mod binary;
mod math;
mod state;
mod vm;

use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    path::Path
};

use crate::api::lua_vm::LuaAPI;

fn main() {
    if env::args().len() > 1 {
        let filename = env::args().nth(1).unwrap();
        match read_file(filename) {
            Ok(data) => {
                let mut ls = state::new_lua_state();
                ls.register("print", print);
                ls.load(data, "test.lua", "b");
                ls.call(0, 0);
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = Vec::new();
    reader.read_to_end(&mut contents)?;
    Ok(contents)
}

fn print(ls: &dyn LuaAPI) -> usize {
    let n_args = ls.top();
    for i in 1..(n_args + 1) {
        if ls.is_string(i) {
            print!("{}", ls.to_string(i));
        } else {
            print!("{}", ls.type_name_str(ls.type_enum_id(i)));
        }
        if i < n_args {
            print!("\t")
        }
    }

    println!();
    0
}

