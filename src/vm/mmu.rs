pub const RAM_SIZE: usize = 0xFFFFFF;

#[derive(Debug)]
pub struct Mmu<'a> {
    ram: &'a mut [u8],
}

#[allow(dead_code)]
impl<'a> Mmu<'a> {
    pub fn load(&mut self, buffer: &[u8]) {
        for (i, x) in buffer.iter().enumerate() {
            self.ram[i] = *x;
        }
    }

    pub fn from_vec(buffer: Vec<u8>) -> Self {
        Mmu {
            ram: Box::leak(buffer.into_boxed_slice()),
        }
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u32, val: u8) {
        self.ram[addr as usize] = val;
    }

    pub fn read_word(&self, addr: u32) -> u16 {
        assert!(addr % 2 == 0, "Memory access not word aligned!");
        ((self.ram[addr as usize] as u16) << 8) + self.ram[addr as usize + 1] as u16
    }

    pub fn write_word(&mut self, addr: u32, val: u16) {
        assert!(addr % 2 == 0, "Memory access not word aligned!");
        self.ram[addr as usize] = ((0xFF00 & val) >> 8) as u8;
        self.ram[addr as usize + 1] = (0xFF & val) as u8;
    }

    pub fn read_long(&self, addr: u32) -> u32 {
        assert!(addr % 2 == 0, "Memory access not word aligned!");
        ((self.ram[addr as usize] as u32) << 24)
            + ((self.ram[addr as usize + 1] as u32) << 16)
            + ((self.ram[addr as usize + 2] as u32) << 8)
            + self.ram[addr as usize + 3] as u32
    }

    pub fn write_long(&mut self, addr: u32, val: u32) {
        assert!(addr % 2 == 0, "Memory access not word aligned!");
        self.ram[addr as usize] = ((0xFF000000 & val) >> 24) as u8;
        self.ram[addr as usize + 1] = ((0x00FF0000 & val) >> 16) as u8;
        self.ram[addr as usize + 2] = ((0x0000FF00 & val) >> 8) as u8;
        self.ram[addr as usize + 3] = (0xFF & val) as u8;
    }

    pub fn get_slice(&self) -> &[u8] {
        self.ram
    }
}

impl<'a> Default for Mmu<'a> {
    fn default() -> Self {
        let rambox = vec![0; RAM_SIZE].into_boxed_slice();
        let ramref = Box::leak(rambox);
        Self { ram: ramref }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_memory_bounds() {
        let mmu = Mmu::default();
        assert_eq!(mmu.ram[0x0000], 0);
        assert_eq!(mmu.ram[0xFFFE], 0);
        mmu.ram[0xDEAD] = 0xAF;
        assert_eq!(mmu.ram[0xDEAD], 0xAF);
    }

    #[test]
    fn test_long_conversions() {
        let mut mmu = Mmu::default();
        mmu.write_long(0x1234, 0xDEADBEEF);
        assert_eq!(mmu.read_long(0x1234), 0xDEADBEEF);

        mmu.write_long(0xABCE, 0x001A02F0);
        assert_eq!(mmu.read_long(0xABCE), 0x001A02F0);
    }

    #[test]
    fn test_word_conversions() {
        let mut mmu = Mmu::default();
        mmu.write_word(0x1234, 0xBEEF);
        assert_eq!(mmu.read_word(0x1234), 0xBEEF);

        mmu.write_word(0xABCE, 0x02F0);
        assert_eq!(mmu.read_word(0xABCE), 0x02F0);
    }
}
