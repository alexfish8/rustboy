use cpu::Cpu;

pub struct Instruction {
    pub name: &'static str,
    pub f: fn(&mut Cpu) -> (),
    pub size: usize, // the number of bytes used to represent this instruction
}


pub fn decode(opcode: u8) -> Instruction {
    let instruction = match opcode {
        0x31    =>
                    Instruction {
                        name: "LXI SP",
                        size: 3,
                        f: lxi,
                    },
        _       => panic!("Opcode not implemented: {:x}", opcode)
    };

    instruction
}

fn lxi(cpu: &mut Cpu) -> () {
    let lower_byte = cpu.memory[cpu.pc() + 1] as u16;
    let higher_byte = cpu.memory[cpu.pc() + 2] as u16;
    let value = (higher_byte << 8) + lower_byte;
    cpu.stack_pointer = value;
    cpu.program_counter += 3;
}


