use super::cpu::Cpu;

impl<'a> Cpu<'a> {
    pub(super) fn exec(&mut self, inst: u16) {
        println!("Instruction: {inst:#018b}");
        match inst {
            0b0000_0000_0000_0000..=0b0000_0010_0000 => self.ori_family(inst),
            0b0000_0010_0011_1100  => self.andi_to_ccr(),
            0b0000_0010_0111_1100 => self.andi_to_sr(),            
            0b0001_0000_0000_0000..=0b0011_1111_1111_1111 => self.move_family(inst),
            0b0100_1110_0111_0001 => self.nop(),
            0b1100_0000_0000_0000..=0b1100_1111_1111_1111 => self.abcd(inst),

            _ => panic!("Unimplemented: {:#018b}", inst)
        }
    }

    /*    ORI    */
    fn ori_family(&mut self, inst: u16) {
        if inst == 0b0000_0000_0011_1100 {
            return self.ori_to_ccr()
        }
        if inst == 0b0000_0000_0111_1100 {
            return self.ori_to_sr()
        }
        self.ori(inst)
    }

    fn ori_to_ccr(&mut self) {
        todo!()
    }

    fn ori_to_sr(&mut self) {
        todo!()
    }

    fn ori(&mut self, _inst: u16) {
        todo!()
    }
    
    /*    ANDI    */
    fn andi_to_ccr(&mut self) {
        let operand = self.fetch_word();
        self.sr &= operand & 0b0000_0000_0001_1111;
    }

    fn andi_to_sr(&mut self) {
        let operand = self.fetch_word();
        self.sr &= operand & 0b0000_0000_0001_1111;
    }

    fn nop(&mut self) {}

    fn move_family(&mut self, inst: u16) {
        let mode = (inst & 0b0000_0001_1100_0000) >> 6;
        if mode == 0b001 {
            return self.movea(inst);
        } 
        self.r#move(inst);    
    }

    fn movea(&mut self, inst: u16) {
        let size = (0b0011_0000_0000_0000 & inst) >> 12;
        assert!((size == 0b11) | (size == 0b10));
        let _dest = (0b0000_1110_0000_0000 & inst) >> 9;
        let _ea = AddressingMode::from(inst);
        todo!()
    }

    fn r#move(&mut self, _inst: u16) {
        todo!()
    }

    fn abcd(&mut self, _inst: u16) {
        todo!()
    }
}

#[repr(u8)]
pub enum AddressingMode {
    DataRegisterDirect(u8) = 0b000, // Dn
    AddressRegisterDirect(u8) = 0b001, // An
    AddressRegisterIndirect(u8) = 0b010, // (An)
    AddressRegisterIndirectPostIncrement(u8) = 0b011, // (An)+
    AddressRegisterIndirectPreDecrement(u8) = 0b100, // -(An)
    AddressRegisterIndirectDisplacement(u8) = 0b101, // (d16,An)
    AddressRegisterIndirectIndex(u8) = 0b110, // (d8,An,Dn)
    Extension(ExtensionMode) = 0b111,
}

impl AddressingMode {
    pub fn from(inst: u16) -> Self {
        let mode = (0b0000_0000_0011_1000 & inst) >> 3;
        let reg = 0b111 & inst;
        match mode {
            0b000 => Self::DataRegisterDirect(reg.try_into().unwrap()),
            0b001 => Self::AddressRegisterDirect(reg.try_into().unwrap()),
            0b010 => Self::AddressRegisterIndirect(reg.try_into().unwrap()),
            0b011 => Self::AddressRegisterIndirectPostIncrement(reg.try_into().unwrap()),
            0b100 => Self::AddressRegisterIndirectPreDecrement(reg.try_into().unwrap()),
            0b101 => Self::AddressRegisterIndirectDisplacement(reg.try_into().unwrap()),
            0b110 => Self::AddressRegisterIndirectIndex(reg.try_into().unwrap()),
            0b111 => {
                match reg {
                    0b000 => Self::Extension(ExtensionMode::Short),
                    0b001 => Self::Extension(ExtensionMode::Long),
                    0b010 => Self::Extension(ExtensionMode::PcRelativeDisplacement),
                    0b011 => Self::Extension(ExtensionMode::PcRelativeIndex),
                    0b100 => Self::Extension(ExtensionMode::Immediate),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
}

pub enum ExtensionMode {
    Short = 0b000, // <addr>.w
    Long = 0b001, // <addr>.l
    PcRelativeDisplacement = 0b010, // d16(PC)
    PcRelativeIndex = 0b011, // (d8, Dn, PC)
    Immediate = 0b100, // #<data>
}