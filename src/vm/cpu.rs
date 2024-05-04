use super::mmu::Mmu;
use crate::types::sign_transmute;

#[derive(Default, Debug)]
pub(super) struct Cpu<'a> {
    pub(super) sr: u16,
    pub(super) pc: usize,
    pub data_registers: [u32; 8],
    pub addr_registers: [u32; 8],
    pub mmu: Mmu<'a>,
}

impl<'a> Cpu<'a> {
    pub fn run(&mut self) {
        loop {
            println!("PC: {:#010x}", self.pc);
            let inst = self.fetch_word();
            self.exec(inst)
        }
    }

    pub fn load(&mut self, buffer: &[u8]) {
        self.mmu.load(buffer);
    }

    pub fn fetch_word(&mut self) -> u16 {
        self.pc += 2;
        self.mmu.read_word(self.pc - 2)
    }

    pub fn fetch_signed_word(&mut self) -> i16 {
        self.pc += 2;
        sign_transmute(self.mmu.read_word(self.pc - 2))
    }

    pub fn fetch_long(&mut self) -> u32 {
        self.pc += 4;
        self.mmu.read_long(self.pc - 4)
    }
}
