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
use lcd::Lcd;

pub struct Clock {

}

const CLOCK_FREQUENCY : f32 = 4.194304e6; // TODO let this be configured by the user

pub struct Gameboy;
impl Gameboy {
    pub fn run(c: Cartridge) -> Result<(), &'static str> {


        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();


        // setup the window context
        // TODO consider wrapping the SDL structures into a unified struct
        let window = video.window("Window title", 800, 600)
                          .position_centered()
                          .opengl()
                          .build()
                          .unwrap();

        let mut renderer = window.renderer()
                                 .accelerated()
                                 .build().unwrap();


        let mut cpu = Cpu::new(c);
        let mut lcd = Lcd::new(renderer);

        let mut cycles = 0;

        while true {
            cpu.step();
            lcd.step();
            // wait some amount of time to handle the clock

        }

        Ok(())
    }




}
