use log::trace;

use crate::{
    types::{AddressingMode, Size, Value},
    util::{get_bits, get_reg, get_size, is_bit_set, is_negative, SizeCoding},
    vm::cpu::Cpu,
    StatusRegister as SR,
};

impl<'a> Cpu<'a> {
    pub(super) fn mul_family(&mut self, inst: u16) {
        if inst >> 4 & 0b11111 == 0b10000 {
            self.abcd(inst)
        }
        match get_bits(inst, 3, 6) {
            0b011000..=0b011111 => self.mulu(inst),
            0b111000..=0b111111 => self.muls(inst),
            0b101000..=0b110001 => self.exg(inst),
            _ => self.and(inst),
        }
    }

    fn mulu(&mut self, _inst: u16) {
        todo!()
    }

    fn muls(&mut self, _inst: u16) {
        todo!()
    }

    fn abcd(&mut self, _inst: u16) {
        todo!()
    }

    fn exg(&mut self, inst: u16) {
        let rx = get_reg(inst, 9);
        let ry = get_reg(inst, 0);
        let mode = get_bits(inst, 3, 5);
        match mode {
            0b01000 => {
                // Data <-> Data
                let vx = self.read_dr(rx);
                let vy = self.read_dr(ry);
                self.write_dr(rx, Size::Long, vy);
                self.write_dr(ry, Size::Long, vx);
                trace!("EXG D{rx} D{ry}");
            }
            0b01001 => {
                // Addr <-> Addr
                let vx = self.read_ar(rx);
                let vy = self.read_ar(ry);
                self.write_ar(rx, vy);
                self.write_ar(ry, vx);
                trace!("EXG A{rx} A{ry}");
            }
            0b10001 => {
                // Data <-> Addr
                let vx = self.read_dr(rx);
                let vy = self.read_ar(ry);
                self.write_dr(rx, Size::Long, vy);
                self.write_ar(ry, vx);
                trace!("EXG D{rx} A{ry}");
            }
            _ => unreachable!(),
        }
    }

    fn and(&mut self, inst: u16) {
        let size = get_size(inst, 6, SizeCoding::Pink);
        let reg = get_reg(inst, 9);
        let val1 = self.read_dr(reg);
        let ea = AddressingMode::from(inst);
        let val2: u32 = self.read_ea(ea, size).into();    
        let result = val1 & val2;        
        if is_bit_set(inst, 8) {
            // Set EA
            trace!("AND.{size} D{reg} {ea} ({val2:#X})");
            self.write_ea(ea, size, Value::Long(result));
        } else {
            // Set Dn
            trace!("AND.{size} {ea} ({val2:#X}) D{reg}");
            self.write_dr(reg, size, result);
        }

        self.write_ccr(SR::N, is_negative(result, size));
        self.write_ccr(SR::Z, result == 0);
        self.write_ccr(SR::V, false);
        self.write_ccr(SR::C, false);
    }
}