use cpu::Cpu;

pub struct Instruction {
    pub name: &'static str,
    pub f: fn(&mut Cpu) -> ()
}
