use std::str;

use std::fs::File;
use std::process;
use std::io::Read;

use instruction;
use instruction::Instruction;
use interrupt::Interrupt;
use cartridge::Cartridge;

const STARTUP_ROM_PATH : &'static str = "startup_rom.bin";
const MEMORY_SIZE: usize = 2 << 16;
const OPCODE_SIZE: usize = 1;


type Register = u16;

pub struct Cpu {
    pub memory: [u8; MEMORY_SIZE], // TODO should this be private, and memory read/written via methods?
    pub program_counter: Register,
    pub stack_pointer: Register,
    a: Register,
    f: Register,
    b: Register,
    c: Register,
    d: Register,
    e: Register,
    h: Register,
    l: Register,
    cartridge: Cartridge
}


impl Cpu {
    pub fn new(c: Cartridge) -> Cpu {
        Cpu {
            memory: [0; MEMORY_SIZE],
            program_counter: 0,
            stack_pointer: 0,
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            cartridge: c,
        }
    }

    pub fn run(&mut self) -> () {
        let startup_rom = Self::load_startup_rom().unwrap();
        assert!(startup_rom.len() <= self.memory.len());

        // map startup rom to memory
        // TODO need to unmap startup ROM at end
        for i in 0..startup_rom.len() {
            self.memory[i] = startup_rom[i];
        }

        loop {

            // TODO need to check to see if interrupts are enabled first
            match self.has_interrupt() {
                Some(interrupt) => self.handle_interrupt(interrupt),
                None            => {}
            };
            self.execute_instruction();
        }
    }

    ///
    /// Return the program counter as a variable of type usize
    pub fn pc(&self) -> usize {
        self.program_counter as usize
    }

    fn has_interrupt(&mut self) -> Option<Interrupt> {
        None // TODO implement this
    }

    fn handle_interrupt(&mut self, int: Interrupt) -> () {
        // TODO implement this
        // maybe the interrupt handling logic should be implemented as a function in the interrupt itself?
    }



    /// Fetches the instruction pointed to by the program counter, decodes it, executes it,
    /// and updates the program counter accordingly
    ///
    /// TODO: Handle interrupts
    fn execute_instruction(&mut self) -> () {
        let opcode = self.memory[self.pc()];
        let instruction = instruction::decode(opcode);

        (instruction.f)(self); // TODO how to handle arguments to instructions?

        // execute instruction
        // update program counter (maybe this should be done by the instruction?)
    }



    fn dummy_instr_execution(cpu: &mut Cpu) -> () {
        println!("Sup son");
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

