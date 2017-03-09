extern crate sdl2;

use self::sdl2::render::Renderer;
use self::sdl2::pixels::Color;
use std::thread;
use std::time::Duration;

pub struct Lcd<'a> {
    renderer: Renderer<'a>,
}

impl<'a> Lcd<'a> {


    pub fn new(renderer: Renderer) -> Lcd {
        Lcd {
            renderer: renderer,
        }
    }


    /// Simulate one clock cycle
    pub fn step(&mut self) -> () {
        println!("LCD step");

        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();
        self.renderer.present();

        thread::sleep(Duration::from_millis(500))

        // for now, we'll just draw the screen
    }
}