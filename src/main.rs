extern crate rustboy;

use rustboy::cartridge::Cartridge;
use rustboy::gameboy::Gameboy;

use std::env;
use std::process;
use std::fs::File;

const STARTUP_ROM_PATH : &'static str = "startup_rom.bin";

fn main() {

    let mut args = env::args();
    args.next();

    let rom_path = match args.next() {
        Some(arg) => arg,
        None      => {
            println!("Need to specify a valid ROM path!");
            process::exit(1);
        }
    };

    let mut f = match File::open(&rom_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file: {}", rom_path);
            process::exit(1);
        }
    };

    let cartridge = match Cartridge::new(&mut f) {
        Ok(c)    => c,
        Err(msg) => {
            println!("Failed to load cartridge: {}", msg);
            process::exit(1);
        }
    };

    //

    println!("Got rom path: {}", rom_path);

    Gameboy::run(cartridge);

}
