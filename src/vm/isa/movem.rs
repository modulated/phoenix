use log::trace;

use crate::{
    types::{AddressingMode, ExtensionMode, Size},
    util::is_bit_set,
    vm::cpu::Cpu,
};

impl<'a> Cpu<'a> {
    pub fn movem(&mut self, inst: u16) {
        match (is_bit_set(inst, 6), is_bit_set(inst, 10)) {
            (false, false) => self.movem_reg_to_mem_word(inst),
            (false, true) => self.movem_mem_to_reg_word(inst),
            (true, false) => self.movem_reg_to_mem_long(inst),
            (true, true) => self.movem_mem_to_reg_long(inst),
        }
    }

    fn movem_mem_to_reg_long(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let mask = self.fetch_word();

        let start = self.get_ea(ea);
        let mut cur = start;

        trace!("MOVEM.l {ea} => [{mask:#06X}]");
        assert!(match ea {
            AddressingMode::DataRegisterDirect(_) => false,
            AddressingMode::AddressRegisterDirect(_) => false,
            AddressingMode::AddressRegisterIndirect(_) => true,
            AddressingMode::AddressRegisterIndirectPostIncrement(_) => true,
            AddressingMode::AddressRegisterIndirectPreDecrement(_) => false,
            AddressingMode::AddressRegisterIndirectDisplacement(_) => true,
            AddressingMode::AddressRegisterIndirectIndex(_) => true,
            AddressingMode::Extension(e) => match e {
                ExtensionMode::Word => true,
                ExtensionMode::Long => true,
                ExtensionMode::PcRelativeDisplacement => true,
                ExtensionMode::PcRelativeIndex => true,
                ExtensionMode::Immediate => false,
            },
        });

        for reg in 0..8 {
            // Data
            if is_bit_set(mask, reg) {
                let val = self.mmu.read_long(cur);
                self.write_dr(reg, Size::Long, val);
                cur += 4;
            }
        }
        let mask = mask >> 8;
        for reg in 0..8 {
            // Addr
            if is_bit_set(mask, reg) {
                let val = self.mmu.read_long(cur);
                self.write_ar(reg, val);
                cur += 4;
            }
        }
    }

    fn movem_reg_to_mem_long(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let mask = self.fetch_word();

        trace!("MOVEM.l [{mask:#X}] => {ea}");

        assert!(match ea {
            AddressingMode::DataRegisterDirect(_) => false,
            AddressingMode::AddressRegisterDirect(_) => false,
            AddressingMode::AddressRegisterIndirect(_) => true,
            AddressingMode::AddressRegisterIndirectPostIncrement(_) => false,
            AddressingMode::AddressRegisterIndirectPreDecrement(_) => true,
            AddressingMode::AddressRegisterIndirectDisplacement(_) => true,
            AddressingMode::AddressRegisterIndirectIndex(_) => true,
            AddressingMode::Extension(e) => match e {
                ExtensionMode::Word => true,
                ExtensionMode::Long => true,
                ExtensionMode::PcRelativeDisplacement => false,
                ExtensionMode::PcRelativeIndex => false,
                ExtensionMode::Immediate => false,
            },
        });

        match ea {
            AddressingMode::AddressRegisterIndirectPreDecrement(_) => {
                // D0 D1 D2 D3 D4 D5 D6 D7 A0 A1 A2 A3 A4 A5 A6 A7
                for reg in 0..8 {
                    // Addr
                    if is_bit_set(mask, reg) {
                        let val = self.read_ar(7 - reg);
                        self.write_ea_long(ea, val);
                    }
                }
                let mask = mask >> 8;
                for reg in 0..8 {
                    // Data
                    if is_bit_set(mask, reg) {
                        let val = self.read_dr(7 - reg);
                        self.write_ea_long(ea, val);
                    }
                }
            }
            _ => {
                // A7 A6 A5 A4 A3 A2 A1 A0 D7 D6 D5 D4 D3 D2 D1 D0
                let mut addr = self.get_ea(ea);
                for reg in 0..8 {
                    // Data
                    if is_bit_set(mask, reg) {
                        let val = self.read_dr(reg);
                        self.mmu.write_long(addr, val);
                        addr += 4;
                    }
                }
                let mask = mask >> 8;
                for reg in 0..8 {
                    // Addr
                    if is_bit_set(mask, reg) {
                        let val = self.read_ar(reg);
                        self.mmu.write_long(addr, val);
                        addr += 4;
                    }
                }
            }
        };
    }

    pub fn movem_mem_to_reg_word(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let mask = self.fetch_word();

        let start = self.get_ea(ea);
        let mut cur = start;

        trace!("MOVEM.w {ea} => [{mask:#06X}]");
        assert!(match ea {
            AddressingMode::DataRegisterDirect(_) => false,
            AddressingMode::AddressRegisterDirect(_) => false,
            AddressingMode::AddressRegisterIndirect(_) => true,
            AddressingMode::AddressRegisterIndirectPostIncrement(_) => true,
            AddressingMode::AddressRegisterIndirectPreDecrement(_) => false,
            AddressingMode::AddressRegisterIndirectDisplacement(_) => true,
            AddressingMode::AddressRegisterIndirectIndex(_) => true,
            AddressingMode::Extension(e) => match e {
                ExtensionMode::Word => true,
                ExtensionMode::Long => true,
                ExtensionMode::PcRelativeDisplacement => true,
                ExtensionMode::PcRelativeIndex => true,
                ExtensionMode::Immediate => false,
            },
        });

        for reg in 0..8 {
            // Data
            if is_bit_set(mask, reg) {
                let val = self.mmu.read_word(cur) as u32;
                self.write_dr(reg, Size::Word, val);
                cur += 2;
            }
        }
        let mask = mask >> 8;
        for reg in 0..8 {
            // Addr
            if is_bit_set(mask, reg) {
                let val = self.mmu.read_word(cur) as u32;
                self.write_ar(reg, val);
                cur += 2;
            }
        }
    }

    fn movem_reg_to_mem_word(&mut self, inst: u16) {
        let ea = AddressingMode::from(inst);
        let mask = self.fetch_word();

        trace!("MOVEM.w [{mask:#X}] => {ea}");

        assert!(match ea {
            AddressingMode::DataRegisterDirect(_) => false,
            AddressingMode::AddressRegisterDirect(_) => false,
            AddressingMode::AddressRegisterIndirect(_) => true,
            AddressingMode::AddressRegisterIndirectPostIncrement(_) => false,
            AddressingMode::AddressRegisterIndirectPreDecrement(_) => true,
            AddressingMode::AddressRegisterIndirectDisplacement(_) => true,
            AddressingMode::AddressRegisterIndirectIndex(_) => true,
            AddressingMode::Extension(e) => match e {
                ExtensionMode::Word => true,
                ExtensionMode::Long => true,
                ExtensionMode::PcRelativeDisplacement => false,
                ExtensionMode::PcRelativeIndex => false,
                ExtensionMode::Immediate => false,
            },
        });

        match ea {
            AddressingMode::AddressRegisterIndirectPreDecrement(_) => {
                // D0 D1 D2 D3 D4 D5 D6 D7 A0 A1 A2 A3 A4 A5 A6 A7
                for reg in 0..8 {
                    // Addr
                    if is_bit_set(mask, reg) {
                        let val = self.read_ar(7 - reg) as u16;
                        self.write_ea_word(ea, val);
                    }
                }
                let mask = mask >> 8;
                for reg in 0..8 {
                    // Data
                    if is_bit_set(mask, reg) {
                        let val = self.read_dr(7 - reg) as u16;
                        self.write_ea_word(ea, val);
                    }
                }
            }
            _ => {
                // A7 A6 A5 A4 A3 A2 A1 A0 D7 D6 D5 D4 D3 D2 D1 D0
                let mut addr = self.get_ea(ea);
                for reg in 0..8 {
                    // Data
                    if is_bit_set(mask, reg) {
                        let val = self.read_dr(reg) as u16;
                        self.mmu.write_word(addr, val);
                        addr += 2;
                    }
                }
                let mask = mask >> 8;
                for reg in 0..8 {
                    // Addr
                    if is_bit_set(mask, reg) {
                        let val = self.read_ar(reg) as u16;
                        self.mmu.write_word(addr, val);
                        addr += 2;
                    }
                }
            }
        };
    }
}
