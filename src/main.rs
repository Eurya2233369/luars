mod binary;
mod vm;

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use binary::{chunk::Prototype, un_dump};

fn main() {
    let path = "/data/data/com.termux/files/home/project/Luars/test/luac.out";

    match read_file(path) {
        Ok(mut content) => {
            let proto = un_dump(&mut content);
            list(proto);
        },
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
