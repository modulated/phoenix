mod cpu;
use cpu::Cpu;
mod isa;
mod mmu;

#[derive(Debug, Default)]
pub struct VM<'a> {
    cpu: Cpu<'a>    
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        let mut out = Self {
            ..Default::default()
        };
        out.cpu.pc = 0x0400;
        out
    }

    pub fn load(&mut self, rom: &[u8]) {
        self.cpu.load(rom);
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
}