extern crate sdl2;

use std::fs::File;
use std::process;
use std::io::Read;

use self::sdl2::pixels::Color;
use std::thread;
use std::time::Duration;
use std::ops::Index;

use cpu::Cpu;
use cartridge::Cartridge;

pub struct Gameboy;
impl Gameboy {

    pub fn run(c: Cartridge) -> Result<(), &'static str> {

        // set up registers and RAM
        // run initial ROM
        // make sure to handle re-mapping of cartridge ROM
        // it should now begin executing
        let startup_rom = Gameboy::load_startup_rom()?;
        let mut cpu = Cpu::new(c);
        cpu.run();
        Ok(())
    }




}
