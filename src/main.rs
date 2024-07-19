mod api;
mod binary;
mod math;
mod state;
mod vm;

use std::{
    env, fs::File, io::{BufReader, Read}, path::Path
};

use crate::api::lua_vm::LuaAPI;

fn main() {
    let filename = env::args().nth(1).unwrap();
    match read_file(filename) {
        Ok(data) => {
            let mut ls = state::new_lua_state();
            ls.load(data, "test.lua", "b");
            ls.call(0, 0);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
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
