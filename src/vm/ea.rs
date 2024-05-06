use super::cpu::Cpu;
use crate::types::{
    sign_extend_16_to_32, sign_extend_8_to_32, AddressingMode, ExtensionMode, Size, Value,
};

impl<'a> Cpu<'a> {
    pub fn get_ea(&mut self, mode: AddressingMode, size: Size) -> Value {
        use Value::*;
        match size {
            Size::Byte => Byte(self.get_ea_byte(mode)),
            Size::Word => Word(self.get_ea_word(mode)),
            Size::Long => Long(self.get_ea_long(mode)),
        }
    }

    pub fn write_ea(&mut self, mode: AddressingMode, size: Size, val: Value) {
        use Value::*;
        match size {
            Size::Byte => match val {
                Byte(v) => self.write_ea_byte(mode, v),
                Word(v) => self.write_ea_byte(mode, v as u8),
                Long(v) => self.write_ea_byte(mode, v as u8),
            },
            Size::Word => match val {
                Byte(v) => self.write_ea_word(mode, v as u16),
                Word(v) => self.write_ea_word(mode, v),
                Long(v) => self.write_ea_word(mode, v as u16),
            },
            Size::Long => match val {
                Byte(v) => self.write_ea_long(mode, v as u32),
                Word(v) => self.write_ea_long(mode, v as u32),
                Long(v) => self.write_ea_long(mode, v),
            },
        }
    }

    pub fn get_ea_byte(&mut self, _mode: AddressingMode) -> u8 {
        todo!()
    }

    pub fn write_ea_byte(&mut self, _mode: AddressingMode, _val: u8) {
        todo!()
    }

    pub fn get_ea_word(&mut self, mode: AddressingMode) -> u16 {
        use AddressingMode::*;
        match mode {
            DataRegisterDirect(reg) => self.read_dr(reg) as u16,
            AddressRegisterDirect(reg) => self.read_ar(reg) as u16,
            AddressRegisterIndirect(reg) => {
                assert!(reg < 7);
                self.mmu.read_word(self.read_ar(reg))
            }
            AddressRegisterIndirectPostIncrement(reg) => {
                let addr = self.read_ar(reg);
                self.increment_ar(reg, 2);
                self.mmu.read_word(addr)
            }
            AddressRegisterIndirectPreDecrement(reg) => {
                self.decrement_ar(reg, 2);
                self.mmu.read_word(self.read_ar(reg))
            }
            AddressRegisterIndirectDisplacement(reg) => {
                assert!(reg < 7);
                let displacement = self.fetch_signed_word();
                let target = (self.read_ar(reg) as i32 + displacement as i32) as u32;
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
                    self.mmu.read_word(addr.into())
                }
                ExtensionMode::Long => {
                    let addr = self.fetch_long();
                    self.mmu.read_word(addr)
                }
                ExtensionMode::PcRelativeDisplacement => {
                    let pc = self.read_pc();
                    let offset = sign_extend_16_to_32(self.fetch_word());
                    let target = (pc as i32 + offset as i32) as u32;
                    self.mmu.read_word(target)
                }
                ExtensionMode::PcRelativeIndex => {
                    let _pc = self.read_pc();
                    // let index = sign_extend_8_to_32(reg);
                    todo!();
                }
                ExtensionMode::Immediate => self.fetch_word(),
            },
        }
    }

    pub fn write_ea_word(&mut self, mode: AddressingMode, val: u16) {
        match mode {
            AddressingMode::DataRegisterDirect(reg) => self.write_dr(reg, val as u32),
            AddressingMode::AddressRegisterIndirect(reg) => {
                assert!(reg < 7);
                self.mmu.write_word(self.read_ar(reg), val);
            }
            AddressingMode::AddressRegisterIndirectPostIncrement(reg) => {
                self.mmu.write_word(self.read_ar(reg), val);
                self.increment_ar(reg, 2);
            }
            AddressingMode::AddressRegisterIndirectPreDecrement(reg) => {
                self.decrement_ar(reg, 2);
                self.mmu.write_word(self.read_ar(reg), val);
            }
            AddressingMode::AddressRegisterIndirectDisplacement(reg) => {
                assert!(reg < 7);
                let displacement = self.fetch_signed_word();
                let target = (self.read_ar(reg) as i32 + displacement as i32) as u32;
                self.mmu.write_word(target, val);
            }
            AddressingMode::AddressRegisterIndirectIndex(reg) => {
                let exword = self.fetch_word();
                let ar = self.read_ar(reg);
                let offset = self.get_index_offset(exword, Size::Long);
                let addr = ar + offset;
                self.mmu.write_word(addr, val)
            }
            AddressingMode::Extension(e) => match e {
                ExtensionMode::Word => {
                    let addr = self.fetch_word();
                    self.mmu.write_word(addr.into(), val);
                }
                ExtensionMode::Long => {
                    let addr = self.fetch_long();
                    self.mmu.write_word(addr, val)
                }
                _ => panic!("Unable to write with this Addressing Mode"),
            },
            _ => panic!("Unable to write with this Addressing Mode"),
        }
    }

    pub fn get_ea_long(&mut self, mode: AddressingMode) -> u32 {
        use AddressingMode::*;
        match mode {
            DataRegisterDirect(reg) => self.read_dr(reg),
            AddressRegisterDirect(reg) => self.read_ar(reg),
            AddressRegisterIndirect(reg) => {
                assert!(reg < 7);
                self.mmu.read_long(self.read_ar(reg))
            }
            AddressRegisterIndirectPostIncrement(reg) => {
                let addr = self.read_ar(reg);
                self.increment_ar(reg, 4);
                self.mmu.read_long(addr)
            }
            AddressRegisterIndirectPreDecrement(reg) => {
                assert!(reg < 7); // TODO: include SP
                self.decrement_ar(reg, 4);
                self.mmu.read_long(self.read_ar(reg))
            }
            AddressRegisterIndirectDisplacement(reg) => {
                assert!(reg < 7);
                let displacement = self.fetch_signed_word();
                let target = (self.read_ar(reg) as i32 + displacement as i32) as u32;
                self.mmu.read_long(target)
            }
            AddressRegisterIndirectIndex(reg) => {
                assert!(reg < 7);
                let exword = self.fetch_word();
                let ar = self.read_ar(reg);
                let offset = self.get_index_offset(exword, Size::Long);
                let addr = ar + offset;
                self.mmu.read_long(addr)
            }
            Extension(ext) => match ext {
                ExtensionMode::Word => {
                    let addr = self.fetch_word();
                    self.mmu.read_long(addr.into())
                }
                ExtensionMode::Long => {
                    let addr = self.fetch_long();
                    self.mmu.read_long(addr)
                }
                ExtensionMode::PcRelativeDisplacement => {
                    let pc = self.read_pc();
                    let offset = sign_extend_16_to_32(self.fetch_word());
                    let target = offset.wrapping_add(pc + 2);
                    self.mmu.read_long(target)
                }
                ExtensionMode::PcRelativeIndex => {
                    let pc = self.read_pc();
                    let exword = self.fetch_word();
                    let offset = self.get_index_offset(exword, Size::Long);
                    dbg_hex::dbg_hex!(offset);
                    let addr = offset.wrapping_add(pc + 2);
                    dbg_hex::dbg_hex!(addr);
                    self.mmu.read_long(addr)
                }
                ExtensionMode::Immediate => self.fetch_long(),
            },
        }
    }

    pub fn write_ea_long(&mut self, mode: AddressingMode, val: u32) {
        match mode {
            AddressingMode::DataRegisterDirect(reg) => self.write_dr(reg, val),
            AddressingMode::AddressRegisterIndirect(reg) => {
                assert!(reg < 7);
                self.mmu.write_long(self.read_ar(reg), val);
            }
            AddressingMode::AddressRegisterIndirectPostIncrement(reg) => {
                self.mmu.write_long(self.read_ar(reg), val);
                self.increment_ar(reg, 4);
            }
            AddressingMode::AddressRegisterIndirectPreDecrement(reg) => {
                self.decrement_ar(reg, 4);
                self.mmu.write_long(self.read_ar(reg), val);
            }
            AddressingMode::AddressRegisterIndirectDisplacement(reg) => {
                assert!(reg < 7);
                let displacement = self.fetch_signed_word();
                let target = (self.read_ar(reg) as i32 + displacement as i32) as u32;
                self.mmu.write_long(target, val);
            }
            AddressingMode::AddressRegisterIndirectIndex(reg) => {
                let exword = self.fetch_word();
                let ar = self.read_ar(reg);
                let offset = self.get_index_offset(exword, Size::Long);
                let addr = ar + offset;
                self.mmu.write_long(addr, val)
            }
            AddressingMode::Extension(e) => match e {
                ExtensionMode::Word => {
                    let addr = self.fetch_word();
                    self.mmu.write_long(addr.into(), val);
                }
                ExtensionMode::Long => {
                    let addr = self.fetch_long();
                    self.mmu.write_long(addr, val)
                }
                _ => panic!("Unable to write with this Addressing Mode"),
            },
            _ => panic!("Unable to write with this Addressing Mode"),
        }
    }

    /// Extension Bit Format
    ///  |F E D C|B A 9 8|7 6 5 4|3 2 1 0|
    ///  |X|A-A-A|B|-----|C-C-C-C-C-C-C-C|
    /// X: Xi register type - 0 for D, 1 for A
    /// A: Xi
    /// B: Xi size - 0 for word, 1 for long
    /// C: 8 bit signed displacement
    fn get_index_offset(&self, word: u16, size: Size) -> u32 {
        let displacement = sign_extend_8_to_32((word & 0b0000_0000_1111_1111) as u8);
        let reg = ((word & 0b0111_0000_0000_0000) >> 12) as u8;
        let index = if (word & 0b1000_0000_0000_0000) == 0 {
            self.read_dr(reg)
        } else {
            self.read_ar(reg)
        };
        let scale = size as u32;
        dbg!(scale);
        let reg_size = (word & 0b0000_1000_0000_0000) >> 11;
        dbg!(reg_size);
        if reg_size == 0 {
            todo!("word reg size?");
        }
        displacement + (index * scale)
    }
}