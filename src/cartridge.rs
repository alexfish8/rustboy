use std::fs::File;
use std::vec::Vec;
use std::io::Read;


pub struct Cartridge {
    data : Vec<u8>
}

impl Cartridge {
    pub fn new(f : &mut File) -> Result<Cartridge, &'static str> {

        let mut buf : Vec<u8> =  Vec::new();
        let num_bytes = match f.read_to_end(&mut buf) {
            Ok(num_bytes) => num_bytes,
            Err(e)           => return Err("Failed to read file")
        };

        Ok(Cartridge {
            data: buf
        })
    }
}
