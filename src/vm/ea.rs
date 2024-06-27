use super::cpu::Cpu;
use crate::types::{AddressingMode, ExtensionMode, Size, Value};
use crate::util::{sign_extend_16_to_32, sign_extend_8_to_32};

impl<'a> Cpu<'a> {
    pub fn get_ea(&mut self, ea: AddressingMode) -> u32 {
        let val = match ea {
            AddressingMode::DataRegisterDirect(_) => unreachable!(),
            AddressingMode::AddressRegisterDirect(_) => unreachable!(),
            AddressingMode::AddressRegisterIndirect(r) => self.read_ar(r),
            AddressingMode::AddressRegisterIndirectPostIncrement(_) => unreachable!(),
            AddressingMode::AddressRegisterIndirectPreDecrement(_) => unreachable!(),
            AddressingMode::AddressRegisterIndirectDisplacement(r) => {
                let displacement = self.fetch_signed_word();
                let val = self.read_ar(r);
                (val as i64 + displacement as i64) as u32
            }
            AddressingMode::AddressRegisterIndirectIndex(reg) => {
                let exword = self.fetch_word();
                let ar = self.read_ar(reg);
                let offset = self.get_index_offset(exword, Size::Byte);
                ar + offset
            }
            AddressingMode::Extension(e) => match e {
                ExtensionMode::Word => self.fetch_word() as u32,
                ExtensionMode::Long => self.fetch_long(),
                ExtensionMode::PcRelativeDisplacement => {
                    let pc = self.read_pc();
                    let offset = sign_extend_16_to_32(self.fetch_word());
                    (pc as i32 + offset as i32) as u32
                }
                ExtensionMode::PcRelativeIndex => {
                    let pc = self.read_pc();
                    let exword = self.fetch_word();
                    let offset = self.get_index_offset(exword, Size::Byte);
                    offset.wrapping_add(pc)
                }
                ExtensionMode::Immediate => unreachable!(),
            },
        };
        val & 0xFFFFFF
    }

    pub fn read_ea(&mut self, ea: AddressingMode, size: Size) -> Value {
        use Value::*;
        match size {
            Size::Byte => Byte(self.read_ea_byte(ea)),
            Size::Word => Word(self.read_ea_word(ea)),
            Size::Long => Long(self.read_ea_long(ea)),
        }
    }

    pub fn write_ea(&mut self, ea: AddressingMode, size: Size, val: Value) {
        match size {
            Size::Byte => self.write_ea_byte(ea, u32::from(val) as u8),
            Size::Word => self.write_ea_word(ea, u32::from(val) as u16),
            Size::Long => self.write_ea_long(ea, u32::from(val)),
        }
    }

    pub fn read_ea_byte(&mut self, ea: AddressingMode) -> u8 {
        use AddressingMode::*;
        match ea {
            DataRegisterDirect(reg) => self.read_dr(reg) as u8,
            AddressRegisterDirect(reg) => self.read_ar(reg) as u8,
            AddressRegisterIndirect(reg) => {
                assert!(reg < 8);
                self.mmu.read_byte(self.read_ar(reg))
            }
            AddressRegisterIndirectPostIncrement(reg) => {
                let addr = self.read_ar(reg);
                self.increment_ar(reg, 1);
                self.mmu.read_byte(addr)
            }
            AddressRegisterIndirectPreDecrement(reg) => {
                self.decrement_ar(reg, 1);
                self.mmu.read_byte(self.read_ar(reg))
            }
            AddressRegisterIndirectDisplacement(reg) => {
                assert!(reg < 8);
                let displacement = self.fetch_signed_word();
                let target = (self.read_ar(reg) as i32 + displacement as i32) as u32;
                self.mmu.read_byte(target)
            }
            AddressRegisterIndirectIndex(reg) => {
                assert!(reg < 8);

                todo!()
            }
            Extension(ext) => match ext {
                ExtensionMode::Word => {
                    // TODO: should only fetch first or last 32KiB of RAM
                    let addr = self.fetch_word();
                    self.mmu.read_byte(addr.into())
                }
                ExtensionMode::Long => {
                    let addr = self.fetch_long();
                    self.mmu.read_byte(addr)
                }
                ExtensionMode::PcRelativeDisplacement => {
                    let pc = self.read_pc();
                    let offset = sign_extend_16_to_32(self.fetch_word());
                    let target = (pc as i32 + offset as i32) as u32;
                    self.mmu.read_byte(target)
                }
                ExtensionMode::PcRelativeIndex => {
                    let _pc = self.read_pc();
                    // let index = sign_extend_8_to_32(reg);
                    todo!();
                }
                ExtensionMode::Immediate => self.fetch_word() as u8,
            },
        }
    }

    pub fn write_ea_byte(&mut self, ea: AddressingMode, val: u8) {
        match ea {
            AddressingMode::DataRegisterDirect(reg) => {
                let val = (self.read_dr(reg) & 0xFFFFFF00) + val as u32;
                self.write_dr(reg, Size::Byte, val);
            }
            AddressingMode::AddressRegisterIndirect(reg) => {
                assert!(reg < 8);
                self.mmu.write_byte(self.read_ar(reg), val);
            }
            AddressingMode::AddressRegisterIndirectPostIncrement(reg) => {
                self.mmu.write_byte(self.read_ar(reg), val);
                self.increment_ar(reg, 1);
            }
            AddressingMode::AddressRegisterIndirectPreDecrement(reg) => {
                self.decrement_ar(reg, 1);
                self.mmu.write_byte(self.read_ar(reg), val);
            }
            AddressingMode::AddressRegisterIndirectDisplacement(reg) => {
                assert!(reg < 8);
                let displacement = self.fetch_signed_word();
                let target = (self.read_ar(reg) as i32 + displacement as i32) as u32;
                self.mmu.write_byte(target, val);
            }
            AddressingMode::AddressRegisterIndirectIndex(reg) => {
                let exword = self.fetch_word();
                let ar = self.read_ar(reg);
                let offset = self.get_index_offset(exword, Size::Long);
                let addr = ar + offset;
                self.mmu.write_byte(addr, val)
            }
            AddressingMode::Extension(e) => match e {
                ExtensionMode::Word => {
                    let addr = self.fetch_word();
                    self.mmu.write_byte(addr.into(), val);
                }
                ExtensionMode::Long => {
                    let addr = self.fetch_long();
                    self.mmu.write_byte(addr, val)
                }
                _ => panic!("Unable to write with this Addressing Mode"),
            },
            _ => panic!("Unable to write with this Addressing Mode"),
        }
    }

    pub fn read_ea_word(&mut self, ea: AddressingMode) -> u16 {
        use AddressingMode::*;
        match ea {
            DataRegisterDirect(reg) => self.read_dr(reg) as u16,
            AddressRegisterDirect(reg) => self.read_ar(reg) as u16,
            AddressRegisterIndirect(reg) => {
                assert!(reg < 8);
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
                assert!(reg < 8);
                let displacement = self.fetch_signed_word();
                let target = (self.read_ar(reg) as i32 + displacement as i32) as u32;
                self.mmu.read_word(target)
            }
            AddressRegisterIndirectIndex(reg) => {
                assert!(reg < 8);
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

    pub fn write_ea_word(&mut self, ea: AddressingMode, val: u16) {
        match ea {
            AddressingMode::DataRegisterDirect(reg) => {
                let val = (self.read_dr(reg) & 0xFFFF0000) + val as u32;
                self.write_dr(reg, Size::Word, val)
            }
            AddressingMode::AddressRegisterDirect(reg) => {
                // TODO: is this breaking? need this hack for ADDQ + SUBQ
                assert!(reg < 8);
                self.write_ar(reg, val as u32)
            }
            AddressingMode::AddressRegisterIndirect(reg) => {
                assert!(reg < 8);
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
                assert!(reg < 8);
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
        }
    }

    pub fn read_ea_long(&mut self, ea: AddressingMode) -> u32 {
        use AddressingMode::*;
        match ea {
            DataRegisterDirect(reg) => self.read_dr(reg),
            AddressRegisterDirect(reg) => self.read_ar(reg),
            AddressRegisterIndirect(reg) => {
                assert!(reg < 8);
                self.mmu.read_long(self.read_ar(reg))
            }
            AddressRegisterIndirectPostIncrement(reg) => {
                let addr = self.read_ar(reg);
                self.increment_ar(reg, 4);
                self.mmu.read_long(addr)
            }
            AddressRegisterIndirectPreDecrement(reg) => {
                assert!(reg < 8);
                self.decrement_ar(reg, 4);
                self.mmu.read_long(self.read_ar(reg))
            }
            AddressRegisterIndirectDisplacement(reg) => {
                assert!(reg < 8);
                let displacement = self.fetch_signed_word();
                let target = (self.read_ar(reg) as i32 + displacement as i32) as u32;
                self.mmu.read_long(target)
            }
            AddressRegisterIndirectIndex(reg) => {
                assert!(reg < 8);
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
                    let addr = offset.wrapping_add(pc + 2);
                    self.mmu.read_long(addr)
                }
                ExtensionMode::Immediate => self.fetch_long(),
            },
        }
    }

    pub fn write_ea_long(&mut self, ea: AddressingMode, val: u32) {
        match ea {
            AddressingMode::DataRegisterDirect(reg) => self.write_dr(reg, Size::Long, val),
            AddressingMode::AddressRegisterDirect(reg) => self.write_ar(reg, val),
            AddressingMode::AddressRegisterIndirect(reg) => {
                assert!(reg < 8);
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
                assert!(reg < 8);
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
        let mut index = if (word & 0b1000_0000_0000_0000) == 0 {
            self.read_dr(reg)
        } else {
            self.read_ar(reg)
        };
        let scale = size as u32;
        let reg_size = (word & 0b0000_1000_0000_0000) >> 11;
        if reg_size == 0 {
            index &= 0xFFFF
        }
        dbg_hex::dbg_hex!(index);
        dbg_hex::dbg_hex!(displacement);
        dbg_hex::dbg_hex!(scale);
        let res = displacement.wrapping_add(index * scale);
        dbg_hex::dbg_hex!(res);
        res
    }
}
