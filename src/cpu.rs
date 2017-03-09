use std::str;

use std::fs::File;
use std::process;
use std::io::Read;

use instruction;
use instruction::Instruction;
use interrupt;
use interrupt::Interrupt;
use cartridge::Cartridge;

const STARTUP_ROM_PATH : &'static str = "startup_rom.bin";
const MEMORY_SIZE: usize = 2 << 16;
const OPCODE_SIZE: usize = 1;


pub type Register8 = u8;

pub struct Register16 {
    pub hi: u8, // the higher byte of the register
    pub lo: u8, // the lower byte of the regisetr
}


///
/// Represents being executed by the CPU, be it an instruction or an interrupt
struct Executable {
    num_cycles: usize, // how many cycles does this executable take to execute fully?
    cycles_executing: usize, // how many cycles has this executable spent executing?
    execute_fn: fn(&mut Cpu) -> (), // what should happen to the CPU when this is finished executing?
}

impl Executable {
    fn from_instruction(instr: Instruction) -> Executable {

        Executable {
            num_cycles: instr.cycles, // TODO this doesn't need to be a function
            cycles_executing: 0,
            execute_fn: instr.f,
        }
    }


    // TODO figure out a way to refactor this struct
    fn from_interrupt(int: Interrupt) -> Executable {
        Executable {
            num_cycles: 0,
            cycles_executing: 0,
            execute_fn: interrupt::derp,
        }
    }
}

impl Register16 {

    ///
    /// Load a 16 bit value into the register, setting the hi and lo registers accordingly
    pub fn load(&mut self, val: u16) -> () {
        self.lo = (val & 0xFF )as u8;
        self.hi = ((val & 0xFF00) >> 2) as u8;
    }

    pub fn to_usize(self) -> usize {
        let upper_byte_val = (self.hi as usize) << 8;
        let lower_byte_val = self.lo as usize;
        upper_byte_val + lower_byte_val
    }

    pub fn new() -> Register16 {
        Register16 {
            hi: 0,
            lo: 0,
        }
    }

}

pub struct Cpu {
    pub memory: [u8; MEMORY_SIZE], // TODO should this be private, and memory read/written via methods?
    pub program_counter: usize,
    pub stack_pointer: Register16,
    pub af: Register16,
    pub bc: Register16,
    pub de: Register16,
    pub hl: Register16,
    cartridge: Cartridge,
    current_executable: Option<Executable>,
}



impl Cpu {
    pub fn new(c: Cartridge) -> Cpu {
        Cpu {
            memory: [0; MEMORY_SIZE],
            program_counter: 0,
            stack_pointer: Register16::new(),
            af: Register16::new(),
            bc: Register16::new(),
            de: Register16::new(),
            hl: Register16::new(),
            cartridge: c,
            current_executable: None,
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
    /// Simulate one clock cycle
    pub fn step(&mut self) -> () {

        // get something new to execute if not doing anything already

        // TODO figure out a more idiomatic way to write this
        let exists = match self.current_executable {
            Some(_) => true,
            None => false,
        };

        if !exists {
            self.current_executable = match self.has_interrupt() {
                Some(interrupt)         => Some(Executable::from_interrupt(interrupt)),
                None                    => {
                    let instruction = instruction::decode(0); // TODO use the actual opcode
                    Some(Executable::from_instruction(instruction))
                },
            };
        }

        let execute_fn = match self.current_executable {
            None                => panic!("Shouldn't get here: we should always be executing something"),
            Some(ref mut executable)    => {
                executable.cycles_executing += 1;
                if executable.cycles_executing == executable.num_cycles {
                    Some(executable.execute_fn)
                } else {
                    None
                }
            }
        };


        // TODO figure out way to consolidate this and the previous function
        if let Some(execute_fn) = execute_fn {
            (execute_fn)(self)
        }

    }

    ///
    /// Return the program counter as a variable of type usize
    pub fn pc(&self) -> usize {
        self.program_counter
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

