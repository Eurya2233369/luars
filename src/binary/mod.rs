use std::rc::Rc;

pub mod chunk;
mod reader;

pub fn un_dump(data: Vec<u8>) -> Rc<chunk::Prototype> {
    let mut reader = reader::Reader::new(data);
    reader.check_header();
    reader.read_byte();
    reader.read_proto("")
}
