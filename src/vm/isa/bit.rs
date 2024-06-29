use crate::types::AddressingMode;
use crate::util::{get_bits, get_reg, get_size, is_bit_set};
use crate::vm::Cpu;

impl<'a> Cpu<'a> {
    pub(crate) fn bit_family(&mut self, inst: u16) {
        match get_bits(inst, 6, 2) {
            0b00 => self.btst(inst),
            0b01 => self.bchg(inst),
            0b10 => self.bclr(inst),
            0b11 => self.bset(inst),
            _ => unreachable!(),
        }
    }

    fn btst(&mut self, inst: u16) {        
        let ea = AddressingMode::from(inst);
        let modulo = if let AddressingMode::DataRegisterDirect(_) = ea {
            32
        } else {
            8
        };
        let val = self.read_ea_byte(ea);

        let res = if is_bit_set(inst, 8) {
            // Reg
            let reg = get_reg(inst, 9);
            let bit = self.read_dr(reg) % modulo;
            is_bit_set(val, bit as u8)
        } else {
            // Imm

        }
    }

    fn bchg(&mut self, _inst: u16) {
        todo!()
    }

    fn bclr(&mut self, _inst: u16) {
        todo!()
    }

    fn bset(&mut self, _inst: u16) {
        todo!()
    }
}
