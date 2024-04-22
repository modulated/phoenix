use super::isa::{AddressingMode, ExtensionMode};

pub const RAM_SIZE: usize = 0xFFFFFF;

#[derive(Debug)]
pub struct Mmu<'a> {
    ram: &'a mut [u8]
}

#[allow(dead_code)]
impl<'a> Mmu<'a> {
    pub fn load(&mut self, buffer: &[u8]) {
        for (i,x) in buffer.iter().enumerate() {
            self.ram[i] = *x;
        }
    }

    pub fn get_word(&mut self, addr: usize) -> u16 {        
        ((self.ram[addr] as u16) << 8) + self.ram[addr + 1] as u16
    }

    pub fn ea_word(&self, addr: AddressingMode) -> u16 {
        match addr {
            AddressingMode::DataRegisterDirect(_reg) => todo!(),
            AddressingMode::AddressRegisterDirect(_reg) => todo!(),
            AddressingMode::AddressRegisterIndirect(_reg) => todo!(),
            AddressingMode::AddressRegisterIndirectPostIncrement(_reg) => todo!(),
            AddressingMode::AddressRegisterIndirectPreDecrement(_reg) => todo!(),
            AddressingMode::AddressRegisterIndirectDisplacement(_reg) => todo!(),
            AddressingMode::AddressRegisterIndirectIndex(_reg) => todo!(),
            AddressingMode::Extension(ext) => {
                match ext {
                    ExtensionMode::Short => todo!(),
                    ExtensionMode::Long => todo!(),
                    ExtensionMode::PcRelativeDisplacement => todo!(),
                    ExtensionMode::PcRelativeIndex => todo!(),
                    ExtensionMode::Immediate => todo!(),
                }
            }
        }
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
}