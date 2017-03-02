use std::str;
use instruction::Instruction;
use interrupt::Interrupt;
use cartridge::Cartridge;

const MEMORY_SIZE: usize = 2 << 16;
const OPCODE_SIZE: usize = 1;


type Register = u16;

pub struct Cpu {
    memory: [u8; MEMORY_SIZE],
    program_counter: Register,
    stack_pointer: Register,
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

        loop {
            match self.has_interrupt() {
                Some(interrupt) => self.handle_interrupt(interrupt),
                None            => {}
            };
            self.execute_instruction();
        }

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
        let op_code = self.memory[self.program_counter as usize];
        let instruction : Instruction = Self::decode_instruction(op_code);

        (instruction.f)(self);

        // execute instruction
        // update program counter (maybe this should be done by the instruction?)
    }



    fn decode_instruction(opcode: u8) -> Instruction {
        Instruction {
            name: "derp",
            f: Self::dummy_instr_execution,
        }
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

