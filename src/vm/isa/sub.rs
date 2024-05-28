use log::trace;

use crate::{
    types::{AddressingMode, Size},
    util::{get_size, is_carry, is_negative, is_overflow, sign_extend_16_to_32, SizeCoding},
    vm::cpu::Cpu,
    StatusRegister as SR,
};

impl<'a> Cpu<'a> {
    pub(super) fn sub_family(&mut self, inst: u16) {
        let reg = ((inst & 0b0000_1110_0000_0000) >> 9) as u8;
        let ea = AddressingMode::from(inst);
        let opmode = (inst & 0b0000_0001_1100_0000) >> 6;
        match opmode {
            0b000 => self.sub_data(reg, ea, Size::Byte),
            0b001 => self.sub_data(reg, ea, Size::Word),
            0b010 => self.sub_data(reg, ea, Size::Long),
            0b011 => self.suba(reg, ea, Size::Word),
            0b100 => self.sub_addr(reg, ea, Size::Byte),
            0b101 => self.sub_addr(reg, ea, Size::Word),
            0b110 => self.sub_addr(reg, ea, Size::Long),
            0b111 => self.suba(reg, ea, Size::Long),
            _ => unreachable!("{inst:018b}"),
        };
    }

    fn suba(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        trace!("SUBA.{size} A{reg} {ea:?}");
        let val1 = self.read_ar(reg);
        let val2 = self.read_ea(ea, size);
        let res = match size {
            Size::Byte => unreachable!(),
            Size::Word => val1.wrapping_sub(sign_extend_16_to_32(u32::from(val2) as u16)),
            Size::Long => val1.wrapping_sub(val2.into()),
        };
        self.write_ar(reg, res);
        set_ccr(self, val1, val2.into(), res, size);
    }

    fn sub_data(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        trace!("SUB {size:?} Dn:{reg} EA:{ea:?}");
        let val1 = self.read_dr(reg);
        let val2 = match size {
            Size::Byte => self.read_ea_byte(ea) as u32,
            Size::Word => self.read_ea_word(ea) as u32,
            Size::Long => self.read_ea_long(ea),
        };
        let res = val1 - val2;
        set_ccr(self, val1, val2, res, size);
        self.write_dr(reg, size, res);
    }

    fn sub_addr(&mut self, reg: u8, ea: AddressingMode, size: Size) {
        trace!("SUB {size:?} EA:{ea:?} Dn:{reg}");
        todo!()
    }

    pub(crate) fn subi(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val = self.read_ea(ea, size);
        trace!("SUBI.{size} {ea:?}: {val}");
        todo!()
    }
}

fn set_ccr(cpu: &mut Cpu, val1: u32, val2: u32, res: u32, size: Size) {
    cpu.write_ccr(SR::X, is_carry(val1, val2, res, size));
    cpu.write_ccr(SR::C, is_carry(val1, val2, res, size));
    cpu.write_ccr(SR::N, is_negative(res, size));
    cpu.write_ccr(SR::Z, res == 0);
    cpu.write_ccr(SR::V, is_overflow(val1, val2, res, size));
}
