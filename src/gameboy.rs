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

        let mut cpu = Cpu::new(c);
        cpu.run();
        Ok(())
    }




}
