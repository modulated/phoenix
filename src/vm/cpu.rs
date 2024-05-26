use super::mmu::Mmu;
use crate::{types::ConditionCode, util::sign_transmute, vm::StatusRegister as SR};

#[derive(Debug)]
pub struct Cpu<'a> {
    sr: u16,
    pc: usize,
    pub(crate) data_registers: [u32; 8],
    pub(crate) addr_registers: [u32; 7],
    usp: u32,
    ssp: u32,
    pub mmu: Mmu<'a>,
}

impl<'a> Default for Cpu<'a> {
    fn default() -> Self {
        Self { 
            sr: 0x2000,
            pc: Default::default(),
            data_registers: Default::default(),
            addr_registers: Default::default(),
            usp: 0x00FFFFFE,
            ssp: 0x01000000,
            mmu: Default::default() 
        }
    }
}

impl<'a> Cpu<'a> {
    pub const STACK: u8 = 7;
    pub fn run(&mut self) {
        loop {
            let inst = self.fetch_word();
            self.exec(inst);
        }
    }

    pub fn step(&mut self) {
        let inst = self.fetch_word();
        self.exec(inst);
    }

    pub fn load(&mut self, buffer: &[u8]) {
        self.mmu.load(buffer);
    }

    pub fn fetch_word(&mut self) -> u16 {
        self.pc += 2;
        self.mmu.read_word(self.pc as u32 - 2)
    }

    pub fn peep_word(&self) -> u16 {        
        self.mmu.read_word(self.pc as u32)
    }

    pub fn fetch_signed_word(&mut self) -> i16 {
        self.pc += 2;
        sign_transmute(self.mmu.read_word(self.pc as u32 - 2))
    }

    pub fn fetch_long(&mut self) -> u32 {
        self.pc += 4;
        self.mmu.read_long(self.pc as u32 - 4)
    }

    pub fn peep_long(&self) -> u32 {        
        self.mmu.read_long(self.pc as u32)
    }

    pub fn push_long(&mut self, val: u32) {
        let new = self.read_ar(Cpu::STACK) - 4;
        self.write_ar(Cpu::STACK, new);
        self.mmu.write_long(new, val);
    }

    pub fn pop_long(&mut self) -> u32 {
        let pc = self.read_ar(Cpu::STACK);
        self.write_ar(Cpu::STACK, pc + 4);
        self.mmu.read_long(pc)
    }

    fn is_supervisor_mode(&self) -> bool {
        (self.sr & 0b0010_0000_0000_0000) == 0b0010_0000_0000_0000
    }

    pub fn decrement_ar(&mut self, reg: u8, by: u32) {
        assert!(reg < 8, "Indexing into non-existant Address Register");
        if reg == 7 {
            if self.is_supervisor_mode() {
                self.ssp -= by;
            } else {
                self.usp -= by;
            }
        } else {
            self.addr_registers[usize::from(reg)] -= by;
        }
    }

    pub fn increment_ar(&mut self, reg: u8, by: u32) {
        assert!(reg < 8, "Indexing into non-existant Address Register");
        if reg == 7 {
            if self.is_supervisor_mode() {
                self.ssp += by;
            } else {
                self.usp += by;
            }
        } else {
            self.addr_registers[usize::from(reg)] += by;
        }
    }

    pub fn read_ar(&self, reg: u8) -> u32 {
        assert!(reg < 8, "Indexing into non-existant Address Register");
        if reg == 7 {
            if self.is_supervisor_mode() {
                self.ssp
            } else {
                self.usp
            }
        } else {
            self.addr_registers[usize::from(reg)]
        }
    }

    pub fn read_dr(&self, reg: u8) -> u32 {
        assert!(reg < 8, "Indexing into non-existant Data Register");
        self.data_registers[usize::from(reg)]
    }

    pub fn write_ar(&mut self, reg: u8, val: u32) {
        assert!(reg < 8, "Indexing into non-existant Address Register");
        if reg == 7 {
            if self.is_supervisor_mode() {
                self.ssp = val;
            } else {
                self.usp = val;
            }
        } else {
            self.addr_registers[usize::from(reg)] = val;
        }
    }

    pub fn write_dr(&mut self, reg: u8, val: u32) {
        assert!(reg < 8, "Indexing into non-existant Data Register");
        self.data_registers[usize::from(reg)] = val;
    }

    pub fn read_pc(&self) -> u32 {
        self.pc.try_into().unwrap()
    }

    pub fn write_pc(&mut self, pc: u32) {
        self.pc = pc.try_into().unwrap();
    }

    pub fn write_sp(&mut self, val: u32) {
        self.write_ar(Cpu::STACK, val);
    }

    pub fn read_sp(&self) -> u32 {
        self.read_ar(Cpu::STACK)
    }
    
    pub fn read_sr(&self) -> u16 {
        self.sr
    }

    pub fn read_ccr(&self, sr: StatusRegister) -> bool {
        (self.sr & sr as u16) != 0
    }

    pub fn write_ccr(&mut self, sr: StatusRegister, val: bool) {
        if val {
            self.sr |= sr as u16;
        } else {
            self.sr &= !(sr as u16);
        }
    }

    pub fn read_ssp(&self) -> u32 {
        self.ssp
    }

    pub fn read_usp(&self) -> u32 {
        self.usp
    }

    pub fn test_cc(&self, cc: ConditionCode) -> bool {
        match cc {
            ConditionCode::True => true,
            ConditionCode::False => false,
            ConditionCode::Higher => !self.read_ccr(SR::C) & !self.read_ccr(SR::Z),
            ConditionCode::LowerOrSame => self.read_ccr(SR::C) | self.read_ccr(SR::Z),
            ConditionCode::CarryClear => !self.read_ccr(SR::C),
            ConditionCode::CarrySet => self.read_ccr(SR::C),
            ConditionCode::NotEqual => !self.read_ccr(SR::Z),
            ConditionCode::Equal => self.read_ccr(SR::Z),
            ConditionCode::OverflowClear => !self.read_ccr(SR::V),
            ConditionCode::OverflowSet => self.read_ccr(SR::V),
            ConditionCode::Plus => !self.read_ccr(SR::N),
            ConditionCode::Minus => self.read_ccr(SR::N),
            ConditionCode::GreaterOrEqual => {
                self.read_ccr(SR::N) & self.read_ccr(SR::V)
                    | !self.read_ccr(SR::N) & !self.read_ccr(SR::V)
            }
            ConditionCode::LessThan => {
                self.read_ccr(SR::N) & !self.read_ccr(SR::V)
                    | !self.read_ccr(SR::N) & self.read_ccr(SR::V)
            }
            ConditionCode::GreatherThan => {
                self.read_ccr(SR::N) & self.read_ccr(SR::V) & !self.read_ccr(SR::Z)
                    | !self.read_ccr(SR::N) & !self.read_ccr(SR::V) & !self.read_ccr(SR::Z)
            }
            ConditionCode::LessOrEqual => {
                self.read_ccr(SR::Z)
                    | self.read_ccr(SR::N) & !self.read_ccr(SR::V)
                    | !self.read_ccr(SR::N) & self.read_ccr(SR::V)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StatusRegister {
    C = 1,
    V = 2,
    Z = 4,
    N = 8,
    X = 16,
}

#[cfg(test)]
mod test_sr_bitlogic {
    use super::{Cpu, StatusRegister as SR};

    #[test]
    fn test_x() {
        let mut cpu = Cpu::default();
        assert!(!cpu.read_ccr(SR::X));
        cpu.write_ccr(SR::X, true);
        assert!(cpu.read_ccr(SR::X));
        cpu.write_ccr(SR::X, false);
        assert!(!cpu.read_ccr(SR::X));
    }

    #[test]
    fn test_c() {
        let mut cpu = Cpu::default();
        assert!(!cpu.read_ccr(SR::C));
        cpu.write_ccr(SR::C, true);
        assert!(cpu.read_ccr(SR::C));
        cpu.write_ccr(SR::C, false);
        assert!(!cpu.read_ccr(SR::C));
    }
}

#[cfg(test)]
mod test_stack {
    use super::Cpu;

    #[test]
    fn test_long_stack() {
        let mut cpu = Cpu::default();
        cpu.write_ar(Cpu::STACK, 0xFFF0);
        cpu.push_long(0xFAFABABA);
        cpu.push_long(0xABBA1050);
        assert_eq!(cpu.pop_long(), 0xABBA1050);
        assert_eq!(cpu.pop_long(), 0xFAFABABA);
    }
}

#[cfg(test)]
mod test_ea_long {
    use crate::types::{AddressingMode::*, ExtensionMode::*};
    use crate::vm::mmu::Mmu;

    use super::Cpu;

    const ADDR_REG: [u32; 7] = [
        0x00001000, 0x000000A0, 0x00000050, 0x33123456, 0x00000000, 0x00000000, 0x0000008C,
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
    /* 0xA0 */ 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x88, 0x18, 0x10, 0x00, 0x0c, 0xff, 0xff, 0xff, 0xe2
    ];

    #[test]
    fn test_address_register_direct() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x00000100,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_long(AddressRegisterDirect(3));
        assert_eq!(ea, (0x33123456));
    }

    #[test]
    fn test_data_register_direct() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x00000100,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_long(DataRegisterDirect(5));
        assert_eq!(ea, (0xD5333333));
    }

    #[test]
    fn test_address_register_indirect() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x00000100,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_long(AddressRegisterIndirect(2));
        assert_eq!(ea, 0x3321837A);
    }

    #[test]
    fn test_address_register_indirect_postincrement() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x00000100,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_long(AddressRegisterIndirectPostIncrement(2));
        assert_eq!(ea, 0x3321837A);
        assert_eq!(cpu.addr_registers[2], 0x00000054);
    }

    #[test]
    fn test_address_register_indirect_predecrement() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x00000100,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_long(AddressRegisterIndirectPreDecrement(2));
        assert_eq!(ea, 0x055689E9);
        assert_eq!(cpu.addr_registers[2], 0x0000004C);
    }

    #[test]
    fn test_address_register_indirect_displacement() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x000000a2,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_long(AddressRegisterIndirectDisplacement(2));
        assert_eq!(ea, 0xDE63FCC4);
    }

    #[test]
    fn test_address_register_indirect_index() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x000000A8,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_long(AddressRegisterIndirectIndex(2));
        assert_eq!(ea, 0xFDBCD6FA);
    }

    #[test]
    fn test_pc_relative_displacement() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x000000AE,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_long(Extension(PcRelativeDisplacement));
        assert_eq!(ea, 0xD07215AB);
    }

    #[test]
    fn test_pc_relative_index() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x000000AE,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_long(Extension(PcRelativeIndex));
        assert_eq!(ea, 0xD07215AB);
    }

    #[test]
    fn test_absolute_word() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x000000A6,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_word(Extension(Word));
        assert_eq!(ea, 0xDC16);
    }

    #[test]
    fn test_absolute_long() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x000000A4,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_long(Extension(Long));
        assert_eq!(ea, 0xDC1651A9);
    }

    #[test]
    fn test_immediate() {
        let mut cpu = Cpu {
            sr: 0x0000,
            pc: 0x000000A4,
            data_registers: DATA_REG,
            addr_registers: ADDR_REG,
            usp: 0,
            ssp: 0,
            mmu: Mmu::from_vec(MEM.to_vec()),
        };
        let ea = cpu.read_ea_long(Extension(Immediate));
        assert_eq!(ea, 0x00000088);
    }
}
