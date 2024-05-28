use log::trace;

use crate::{
    types::{AddressingMode, Size},
    util::{get_reg, get_size, is_bit_set, is_carry, is_negative, is_overflow, sign_extend_16_to_32, SizeCoding},
    vm::cpu::Cpu,
    StatusRegister as SR,
};

impl<'a> Cpu<'a> {
    pub(super) fn cmp_family(&mut self, inst: u16) {
		if (inst >> 3) & 0b111 == 0b001 {
			return self.cmpm(inst)
		}

		if (inst >> 6) & 0b11 == 0b11 {
			return self.cmpa(inst)
		}

		if is_bit_set(inst, 8) {
			self.eor(inst)
		} else {
			self.cmp(inst)
		}
	}

	fn cmpm(&mut self, _inst: u16) {
		todo!()
	}

	fn cmpa(&mut self, inst: u16) {
		let reg = get_reg(inst, 9);
		let dest = self.read_ar(reg);
		let ea = AddressingMode::from(inst);

		let (size, src) = if is_bit_set(inst, 8) {
			(Size::Long, self.read_ea_long(ea))
		} else {
			(Size::Word, sign_extend_16_to_32(self.read_ea_word(ea)))
		};
		let result = dest.wrapping_sub(src);

		trace!("CMPA.{size} {ea} ({src:#X}) A{reg}");
		self.write_ccr(SR::N, is_negative(result, size));
		self.write_ccr(SR::Z, result == 0);
		self.write_ccr(SR::V, is_overflow(dest, src, result, size));		
		self.write_ccr(SR::C, is_carry(dest, src, result, size));
	}

	fn cmp(&mut self, inst: u16) {
		let size = get_size(inst, 6, SizeCoding::Pink);
		let reg = get_reg(inst, 9);
		let dest = self.read_dr(reg);
		let ea = AddressingMode::from(inst);
		let src: u32 = self.read_ea(ea, size).into();
		let result = dest.wrapping_sub(src);

		trace!("CMP.{size} {ea} ({src:#X}) D{reg}");
		self.write_ccr(SR::N, is_negative(result, size));
		self.write_ccr(SR::Z, result == 0);
		self.write_ccr(SR::V, is_overflow(dest, src, result, size));		
		self.write_ccr(SR::C, is_carry(dest, src, result, size));
	}

	fn eor(&mut self, _inst: u16) {
		todo!()
	}
}