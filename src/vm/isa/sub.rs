use log::trace;

use crate::{
    types::{AddressingMode, Size}, util::{get_size, SizeCoding}, vm::cpu::Cpu
};

impl<'a> Cpu<'a> {
    pub(super) fn sub_family(&mut self, inst: u16) {
        let reg = ((inst & 0b0000_1110_0000_0000) >> 9) as u8;
        let ea = AddressingMode::from(inst);
        let opmode = (inst & 0b0000_0001_1100_0000) >> 6;
        match opmode {
            0b000 => self.sub_dn(reg, ea, Size::Byte),
            0b001 => self.sub_dn(reg, ea, Size::Word),
            0b010 => self.sub_dn(reg, ea, Size::Long),
            0b011 => self.suba(reg, ea, Size::Word),
            0b100 => self.sub_ea(reg, ea, Size::Byte),
            0b101 => self.sub_ea(reg, ea, Size::Word),
            0b110 => self.sub_ea(reg, ea, Size::Long),
            0b111 => self.suba(reg, ea, Size::Long),
            _ => unreachable!(),
        };
    }

    fn suba(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        trace!("SUBA {size:?} An:{reg} EA:{ea:?}");
        todo!()
    }

    fn sub_dn(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        trace!("SUB {size:?} Dn:{reg} EA:{ea:?}");
        let val = match size {
            Size::Byte => self.read_ea_byte(ea) as u32,
            Size::Word => self.read_ea_word(ea) as u32,
            Size::Long => self.read_ea_long(ea),
        };
        let res = self.read_dr(reg) - val;
        // TODO: status flag
        self.write_dr(reg, res);
    }

    fn sub_ea(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        trace!("SUB {size:?} EA:{ea:?} Dn:{reg}");
        todo!()
    }

    pub (crate) fn subi(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val = self.read_ea(ea, size);
        trace!("SUBI.{size} {ea:?}: {val}");
        todo!()
    }
}
