mod cpu;
use cpu::Cpu;
mod ea;
mod isa;
mod mmu;

#[derive(Debug, Default)]
pub struct VM<'a> {
    cpu: Cpu<'a>,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn load(&mut self, rom: &[u8]) {
        self.cpu.load(rom);
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.cpu.write_pc(pc);
    }
}
