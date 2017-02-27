use std::fs::File;
use std::process;
use std::io::Read;

use cartridge::Cartridge;

const STARTUP_ROM_PATH : &'static str = "startup_rom.bin";

pub struct Gameboy;
impl Gameboy {

    pub fn run(c: &Cartridge) -> () {

        // set up registers and RAM
        // run initial ROM
        // make sure to handle re-mapping of cartridge ROM
        // it should now begin executing
        let startup_rom = Gameboy::load_startup_rom();

        


    }

    fn load_startup_rom() -> Result<Vec<u8>, &'static str> {
        let mut f = match File::open(STARTUP_ROM_PATH) {
            Ok(f) => f,
            Err(e) => {
                println!("Failed to open file: {}", STARTUP_ROM_PATH);
                process::exit(1);
            }
        };

        let mut buf = Vec::new();

        let num_bytes = match f.read_to_end(&mut buf) {
            Ok(num_bytes) => num_bytes,
            Err(e)           => return Err("Failed to read file")
        };

        Ok(buf)

    }

}
