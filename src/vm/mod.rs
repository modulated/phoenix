mod cpu;

use cpu::Cpu;

pub use self::cpu::StatusRegister;
mod ea;
mod isa;
mod mmu;

#[derive(Debug, Default)]
pub struct VM<'a> {
    pub cpu: Cpu<'a>,
    pub mem_cursor: usize,
    pub inst_time: u128,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self {
            inst_time: 1,
            ..Default::default()
        }
    }

    pub fn load(&mut self, rom: &[u8]) {
        self.cpu.load(rom);
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }

    pub fn step(&mut self) {
        let start = std::time::Instant::now();
        self.cpu.step();
        let now = std::time::Instant::now();
        self.inst_time = (now - start).as_nanos();
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.cpu.write_pc(pc);
    }

    pub fn set_sp(&mut self, sp: u32) {
        self.cpu.write_ar(Cpu::STACK, sp);
    }

    pub fn read_pc(&self) -> u32 {
        self.cpu.read_pc()
    }

    pub fn read_ccr(&self, sr: StatusRegister) -> bool {
        self.cpu.read_ccr(sr)
    }

    pub fn read_ssp(&self) -> u32 {
        self.cpu.read_ssp()
    }

    pub fn read_usp(&self) -> u32 {
        self.cpu.read_usp()
    }

    pub fn read_ar(&self) -> &[u32] {
        self.cpu.addr_registers.as_slice()
    }

    pub fn read_dr(&self) -> &[u32] {
        self.cpu.data_registers.as_slice()
    }
}
