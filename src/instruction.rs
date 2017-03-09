use cpu::Cpu;
use cpu::{Register8, Register16};

pub struct Instruction {
    pub name: &'static str,
    pub f: fn(&mut Cpu) -> (),
    pub size: usize, // the number of bytes used to represent this instruction
    pub cycles: usize, // the number of cycles the instruction takes to execute
}



// todo set up a macro for this
// todo consider coming up with a more clever decoding scheme
pub fn decode(opcode: u8) -> Instruction {
    let instruction = match opcode {

        0x21    =>
                    Instruction {
                        name: "LXI H",
                        size: 3,
                        f: lxi_h,
                        cycles: 12,
                    },

        0x31    =>
                    Instruction {
                        name: "LXI SP",
                        size: 3,
                        f: lxi_sp,
                        cycles: 12,
                    },

        0xAF    =>
                    Instruction {
                        name: "XOR A",
                        size: 1,
                        f: xor_a,
                        cycles: 4,
                    },



        _       => panic!("Opcode not implemented: {:x}", opcode)

    };

    instruction
}

fn lxi_h(cpu: &mut Cpu) -> () {
    lxi(&cpu.memory, &mut cpu.program_counter, &mut cpu.hl);
}


fn lxi_sp(cpu:&mut Cpu) -> () {
    lxi(&cpu.memory, &mut cpu.program_counter, &mut cpu.stack_pointer)
}

fn derp(cpu: &mut Cpu) -> () {

}

///
/// Load a 16-bit value into the register reg and update the program counter accordingly
fn lxi(mem: &[u8], pc: &mut usize, reg: &mut Register16) -> () {
    let current_address = *pc as usize;
    let lower_byte = mem[current_address+ 1] as u16;
    let higher_byte = mem[current_address + 2] as u16;
    let value = (higher_byte << 8) + lower_byte;
    reg.load(value);
    *pc += 3;
}

fn xor_a(cpu: &mut Cpu) -> () {
    cpu.af.hi ^= cpu.af.hi;
    // flags set = Z_0_0_0
    cpu.af.lo = 0x80; // lower 4 bits are always 0
    cpu.program_counter += 1;
}





