pub mod chunk;
mod reader;

pub fn un_dump(data: &mut Vec<u8>) -> chunk::Prototype {
    let mut reader = reader::Reader::new(data);
    reader.check_header();
    reader.read_byte();
    reader.read_proto("")
}
