use super::mmu::Mmu;

#[allow(dead_code)] // TODO: remove
#[derive(Default,Debug)]
pub(super) struct Cpu<'a> {
    pub(super) sr: u16,
    pub(super) pc: usize,
    d0: u32,
    d1: u32,
    d2: u32,
    d3: u32,
    d4: u32,
    d5: u32,
    d6: u32,
    d7: u32,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    usp: usize,
    ssp: usize,   
    mmu: Mmu<'a>, 
}

impl<'a> Cpu<'a> {
    

    pub fn run(&mut self) {
        loop {
            println!("PC: {:#022x}",self.pc);            
            let inst = self.mmu.get_word(self.pc);
            self.pc += 2;
            self.exec(inst)
        }
    }

    pub fn load(&mut self, buffer: &[u8]) {
        self.mmu.load(buffer);
    }

    pub fn fetch_word(&mut self) -> u16 {
        self.pc += 2;
        self.mmu.get_word(self.pc - 2)
    }
}