use log::trace;

use crate::{
    types::{AddressingMode, Size, Value},
    util::{
        get_bits, get_reg, get_size, is_bit_set, is_carry, is_negative, is_overflow,
        sign_extend_16_to_32, sign_extend_8_to_32, SizeCoding,
    },
    vm::cpu::Cpu,
    StatusRegister as SR,
};

impl<'a> Cpu<'a> {
    pub fn add_family(&mut self, inst: u16) {
        if get_bits(inst, 4, 2) == 0 && is_bit_set(inst, 8) {
            return self.addx(inst);
        }

        if get_bits(inst, 6, 2) == 0b11 {
            return self.adda(inst);
        }

        self.add(inst)
    }

    fn addx(&mut self, inst: u16) {
        if is_bit_set(inst, 3) {
            self.addx_addr(inst);
        } else {
            self.addx_data(inst);
        }
    }

    fn addx_data(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let reg1 = get_reg(inst, 9);
        let reg2 = get_reg(inst, 0);
        let val1 = self.read_dr(reg1);
        let val2 = self.read_dr(reg2);

        let res = match size {
            Size::Byte => sign_extend_8_to_32(
                (val1 as u8)
                    .wrapping_add(val2 as u8)
                    .wrapping_add(self.read_ccr(SR::X) as u8),
            ),
            Size::Word => sign_extend_16_to_32(
                (val1 as u16)
                    .wrapping_add(val2 as u16)
                    .wrapping_add(self.read_ccr(SR::X) as u16),
            ),
            Size::Long => val1
                .wrapping_add(val2)
                .wrapping_add(self.read_ccr(SR::X) as u32),
        };

        self.write_dr(reg1, size, res);

        trace!("ADDX.{} D{}, D{}", size, reg1, reg2);
        add_set_ccr(self, val1, val2, res, size);
    }

    fn addx_addr(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let reg1 = get_reg(inst, 9);
        let reg2 = get_reg(inst, 0);
        let ea1 = AddressingMode::AddressRegisterIndirectPreDecrement(reg1);
        let ea2 = AddressingMode::AddressRegisterIndirectPreDecrement(reg2);
        let val1: u32 = self.read_ea(ea1, size).into();
        let val2: u32 = self.read_ea(ea2, size).into();

        let res = val1
            .wrapping_add(val2)
            .wrapping_add(self.read_ccr(SR::X) as u32);

        trace!("ADDX.{} -(A{}), -(A{})", size, reg1, reg2);
        add_set_ccr(self, val1, val2, res, size); // TODO - flags not right
    }

    fn adda(&mut self, inst: u16) {
        let size = if is_bit_set(inst, 8) {
            Size::Long
        } else {
            Size::Word
        };
        let ea = AddressingMode::from(inst);
        let val = match size {
            Size::Word => {
                let v = self.read_ea_word(ea);
                sign_extend_16_to_32(v)
            }
            Size::Long => self.read_ea_long(ea),
            Size::Byte => unreachable!(),
        };
        let reg = get_reg(inst, 9);
        let addr = self.read_ar(reg);
        let res = addr.wrapping_add(val);
        trace!("ADDA.{size} {ea}, A{reg}");
        self.write_ar(reg, res);
    }

    fn add(&mut self, inst: u16) {
        if is_bit_set(inst, 8) {
            self.add_addr(inst);
        } else {
            self.add_data(inst);
        }
    }

    fn add_addr(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let dreg = get_reg(inst, 9);
        let val1 = self.read_dr(dreg);
        let ea = AddressingMode::from(inst);
        let val2 = self.read_ea(ea, size).into();
        let res = match size {
            Size::Byte => Value::Byte((val1 as u8).wrapping_add(val2 as u8)),
            Size::Word => Value::Word((val1 as u16).wrapping_add(val2 as u16)),
            Size::Long => Value::Long((val1).wrapping_add(val2)),
        };
        self.write_ea(ea, size, res);
        trace!("ADD D{} {}", dreg, ea);
        add_set_ccr(self, val1, val2, res.into(), size);
    }

    fn add_data(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let dreg = get_reg(inst, 9);
        let val1 = self.read_dr(dreg);
        let ea = AddressingMode::from(inst);
        let val2 = self.read_ea(ea, size).into();
        let res = match size {
            Size::Byte => Value::Byte((val1 as u8).wrapping_add(val2 as u8)),
            Size::Word => Value::Word((val1 as u16).wrapping_add(val2 as u16)),
            Size::Long => Value::Long((val1).wrapping_add(val2)),
        };
        self.write_dr(dreg, size, res.into());
        trace!("ADD {} D{}", ea, dreg);
        add_set_ccr(self, val1, val2, res.into(), size);
    }

    pub(crate) fn addi(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val: u32 = self.read_ea(ea, size).into();
        let imm = match size {
            Size::Byte => (self.fetch_word() as u8) as u32,
            Size::Word => self.fetch_word() as u32,
            Size::Long => self.fetch_long(),
        };
        let res = match size {
            Size::Byte => Value::Byte((imm as u8).wrapping_add(val as u8)),
            Size::Word => Value::Word((imm as u16).wrapping_add(val as u16)),
            Size::Long => Value::Long(imm.wrapping_add(val)),
        };
        trace!("ADDI.{size} {imm:#X} {ea}");
        self.write_ea(ea, size, res);
        add_set_ccr(self, val, imm, res.into(), size);
    }

    pub(crate) fn addq(&mut self, inst: u16) {
        let data = get_bits(inst, 9, 3);
        let imm = if data == 0 { 8 } else { data as u8 };
        let size = get_size(inst, 6, SizeCoding::Pink);
        let ea = AddressingMode::from(inst);
        let val = self.read_ea(ea, size);
        let res = val + imm;
        trace!("ADDQ.{size} {imm}, {ea} ({val:X})");
        self.write_ea(ea, size, res);

        add_set_ccr(self, val.into(), imm.into(), res.into(), size);
    }
}

fn add_set_ccr(cpu: &mut Cpu, val1: u32, val2: u32, res: u32, size: Size) {
    cpu.write_ccr(SR::X, is_carry(val1, val2, res, size));
    cpu.write_ccr(SR::C, is_carry(val1, val2, res, size));
    cpu.write_ccr(SR::N, is_negative(res, size));
    cpu.write_ccr(SR::Z, res == 0);
    cpu.write_ccr(SR::V, is_overflow(val1, val2, res, size));
}
