use crate::types::{sign_extend_16_to_32, AddressingMode, ExtensionMode, Size, Value};

use super::cpu::Cpu;

impl<'a> Cpu<'a> {
    pub fn get_ea(&mut self, mode: AddressingMode, size: Size) -> Value {
        use Value::*;
        match size {
            Size::Byte => Byte(self.get_ea_byte(mode)),
            Size::Word => Word(self.get_ea_word(mode)),
            Size::Long => Long(self.get_ea_long(mode)),
        }
    }

    pub fn get_ea_byte(&mut self, _mode: AddressingMode) -> u8 {
        todo!()
    }
    pub fn get_ea_word(&mut self, mode: AddressingMode) -> u16 {
        use AddressingMode::*;
        match mode {
            DataRegisterDirect(reg) => self.data_registers[reg as usize] as u16,
            AddressRegisterDirect(reg) => self.addr_registers[reg as usize] as u16,
            AddressRegisterIndirect(reg) => {
                assert!(reg < 7);
                self.mmu
                    .read_word(self.addr_registers[reg as usize] as usize)
            }
            AddressRegisterIndirectPostIncrement(reg) => {
                assert!(reg < 7); // TODO: include SP
                let addr = self.addr_registers[reg as usize];
                dbg!(addr);
                self.addr_registers[reg as usize] += 2;
                dbg!(self.addr_registers[reg as usize]);
                self.mmu.read_word(addr as usize)
            }
            AddressRegisterIndirectPreDecrement(reg) => {
                assert!(reg < 7); // TODO: include SP
                self.addr_registers[reg as usize] -= 2;
                self.mmu
                    .read_word(self.addr_registers[reg as usize] as usize)
            }
            AddressRegisterIndirectDisplacement(reg) => {
                assert!(reg < 7);
                let displacement = self.fetch_signed_word();
                let target =
                    (self.addr_registers[reg as usize] as i32 + displacement as i32) as usize;
                self.mmu.read_word(target)
            }
            AddressRegisterIndirectIndex(reg) => {
                assert!(reg < 7);
                // let index = sign_extend_8_to_32(self.fetch_word() & 0x00FF);
                todo!()
            }
            Extension(ext) => match ext {
                ExtensionMode::Word => {
                    // TODO: should only fetch first or last 32KiB of RAM
                    let addr = self.fetch_word();
                    self.mmu.read_word(addr as usize)
                }
                ExtensionMode::Long => {
                    // TODO: should this be unreachable?
                    unreachable!()
                }
                ExtensionMode::PcRelativeDisplacement => {
                    let pc = self.pc;
                    let offset = sign_extend_16_to_32(self.fetch_word());
                    let target = (pc as i32 + offset as i32) as usize;
                    self.mmu.read_word(target)
                }
                ExtensionMode::PcRelativeIndex => {
                    let _pc = self.pc;
                    // let index = sign_extend_8_to_32(reg);
                    todo!();
                }
                ExtensionMode::Immediate => self.fetch_word(),
            },
        }
    }

    pub fn get_ea_long(&mut self, mode: AddressingMode) -> u32 {
        use AddressingMode::*;
        match mode {
            DataRegisterDirect(reg) => self.data_registers[reg as usize],
            AddressRegisterDirect(reg) => self.addr_registers[reg as usize],
            AddressRegisterIndirect(reg) => {
                assert!(reg < 7);
                self.mmu
                    .read_long(self.addr_registers[reg as usize] as usize)
            }
            AddressRegisterIndirectPostIncrement(reg) => {
                assert!(reg < 7); // TODO: include SP
                let addr = self.addr_registers[reg as usize];
                dbg!(addr);
                self.addr_registers[reg as usize] += 4;
                dbg!(self.addr_registers[reg as usize]);
                self.mmu.read_long(addr as usize)
            }
            AddressRegisterIndirectPreDecrement(reg) => {
                assert!(reg < 7); // TODO: include SP
                self.addr_registers[reg as usize] -= 4;
                self.mmu
                    .read_long(self.addr_registers[reg as usize] as usize)
            }
            AddressRegisterIndirectDisplacement(reg) => {
                assert!(reg < 7);
                let displacement = self.fetch_signed_word();
                let target =
                    (self.addr_registers[reg as usize] as i32 + displacement as i32) as usize;
                self.mmu.read_long(target)
            }
            AddressRegisterIndirectIndex(reg) => {
                assert!(reg < 7);
                // let index = sign_extend_8_to_32(self.fetch_word() & 0x00FF);
                todo!()
            }
            Extension(ext) => match ext {
                ExtensionMode::Word => {
                    unreachable!();
                    // Word(self.fetch_word()) // TODO: should only fetch first or last 32KiB of RAM
                }
                ExtensionMode::Long => {
                    let addr = self.fetch_long();
                    self.mmu.read_long(addr as usize)
                }
                ExtensionMode::PcRelativeDisplacement => {
                    let pc = self.pc;
                    let offset = sign_extend_16_to_32(self.fetch_word());
                    let target = (pc as i32 + offset as i32) as usize;
                    self.mmu.read_long(target)
                }
                ExtensionMode::PcRelativeIndex => {
                    let _pc = self.pc;
                    // let index = sign_extend_8_to_32(reg);
                    todo!();
                }
                ExtensionMode::Immediate => self.fetch_long(),
            },
        }
    }
}

#[cfg(test)]
mod test_ea_long {
    use crate::types::{AddressingMode::*, ExtensionMode::*};
    use crate::vm::mmu::Mmu;

    use super::Cpu;

    const ADDR_REG: [u32; 8] = [
        0x00001000, 0x000000A0, 0x00000050, 0x33123456, 0x00000000, 0x00000000, 0x0000008C,
        0x000000A0,
    ];
    const DATA_REG: [u32; 8] = [
        0x12345678, 0x00000004, 0x00000001, 0xFF00FF00, 0x00FF00FF, 0xD5333333, 0x88888888,
        0x00000000,
    ];

    #[rustfmt::skip]
    const MEM: [u8; 176] = [
    /* 0x00 */ 0x8d, 0x3a, 0xa8, 0xcb, 0x7d, 0x31, 0x5e, 0xa1, 0x93, 0xa5, 0x61, 0x45, 0x00, 0x00, 0x00, 0x80, 
    /* 0x10 */ 0xa5, 0x98, 0xad, 0xc8, 0xb9, 0xa0, 0xc3, 0xc8, 0x17, 0x2b, 0x9e, 0xc8, 0x9b, 0xb2, 0x70, 0xff, 
    /* 0x20 */ 0xaa, 0x2d, 0x13, 0x31, 0xc1, 0x34, 0xd7, 0xfd, 0x18, 0x13, 0xcc, 0x01, 0x53, 0xdb, 0xfb, 0x7b, 
    /* 0x30 */ 0x1c, 0xdb, 0xa6, 0x7b, 0x19, 0xf6, 0xaa, 0xfe, 0x59, 0x76, 0x0c, 0x87, 0x75, 0x04, 0x48, 0x57, 
    /* 0x40 */ 0x16, 0xe0, 0x92, 0xb5, 0x96, 0x0d, 0x0f, 0xd8, 0xfd, 0xc7, 0xb6, 0x82, 0x05, 0x56, 0x89, 0xe9, 
    /* 0x50 */ 0x33, 0x21, 0x83, 0x7a, 0x50, 0xe2, 0xee, 0x3e, 0xdb, 0xf6, 0xe0, 0x0f, 0xde, 0x63, 0xfc, 0xc4, 
    /* 0x60 */ 0x1d, 0x48, 0x52, 0x3f, 0x28, 0x36, 0x29, 0xaa, 0x5d, 0x66, 0xd9, 0x41, 0x7c, 0x33, 0x62, 0xb9, 
    /* 0x70 */ 0xfd, 0xbc, 0xd6, 0xfa, 0xa2, 0x32, 0xb8, 0xd8, 0xa0, 0x13, 0x1c, 0xba, 0x1b, 0xef, 0x93, 0x96, 
    /* 0x80 */ 0x75, 0x68, 0x19, 0xf3, 0x2d, 0x13, 0xba, 0x27, 0xdc, 0x16, 0x51, 0xa9, 0x65, 0xff, 0xfd, 0x86, 
    /* 0x90 */ 0xd7, 0x04, 0xd0, 0x72, 0x15, 0xab, 0x8b, 0x89, 0xe3, 0x4d, 0x86, 0xf2, 0x00, 0x00, 0x00, 0x10, 
    /* 0xA0 */ 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x88, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x88
    ];

    #[test]
    fn test_address_register_direct() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x00000100,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.get_ea_long(AddressRegisterDirect(3));
        assert_eq!(ea, (0x33123456));
    }

    #[test]
    fn test_data_register_direct() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x00000100,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.get_ea_long(DataRegisterDirect(5));
        assert_eq!(ea, (0xD5333333));
    }

    #[test]
    fn test_address_register_indirect() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x00000100,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.get_ea_long(AddressRegisterIndirect(2));
        assert_eq!(ea, (0x3321837A));
    }

    #[test]
    fn test_address_register_indirect_postincrement() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x00000100,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.get_ea_long(AddressRegisterIndirectPostIncrement(2));
        assert_eq!(ea, (0x3321837A));
        assert_eq!(cpu.addr_registers[2], 0x00000054);
    }

    #[test]
    fn test_address_register_indirect_predecrement() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x00000100,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.get_ea_long(AddressRegisterIndirectPreDecrement(2));
        assert_eq!(ea, (0x055689E9));
        assert_eq!(cpu.addr_registers[2], 0x0000004C);
    }

    #[test]
    fn test_address_register_indirect_displacement() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x000000a2,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.get_ea_long(AddressRegisterIndirectDisplacement(2));
        assert_eq!(ea, (0xDE63FCC4));
    }

    #[test]
    fn test_absolute() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x000000A4,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.get_ea_long(Extension(Long));
        assert_eq!(ea, (0xDC1651A9));
    }

    #[test]
    fn test_immediate() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x000000A4,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.get_ea_long(Extension(Immediate));
        assert_eq!(ea, 0x00000088);
    }
}
