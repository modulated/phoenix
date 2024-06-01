use log::trace;

use crate::{
    types::{AddressingMode, ExtensionMode, Size},
    util::is_bit_set,
    vm::cpu::Cpu,
};

	impl<'a> Cpu<'a> {
	pub fn movem_long(&mut self, inst: u16) {
		let ea = AddressingMode::from(inst);
		let mask = self.fetch_word();

		let start = self.get_ea(ea);
		let mut cur = start;

		if is_bit_set(inst, 10) {
			// Memory to Register
			trace!("MOVEM.l {ea:?}: {start} => {mask:018b}");
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
		} else {
			// Register to Memory
			trace!(
				"{}: MOVEM.l {mask:#X} => {ea:?}: {start:#X}",
				self.read_pc()
			);
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

			for reg in 0..8 {
				// D
				if is_bit_set(mask, reg) {
					let val = self.read_dr(reg);
					self.mmu.write_long(cur, val);
					cur += 4;
				}
			}
			let mask = mask >> 8;
			for reg in 0..8 {
				// A
				if is_bit_set(mask, reg) {
					let val = self.read_ar(reg);
					self.mmu.write_long(cur, val);
					cur += 4;
				}
			}
			// TODO - finish logic
		}
	}

	pub fn movem_word(&mut self, inst: u16) {
		let ea = AddressingMode::from(inst);
		let mask = self.fetch_word();

		let start = self.get_ea(ea);
		let mut cur = start;

		if is_bit_set(inst, 10) {
			// Memory to Register
			trace!("MOVEM.w {ea:?}: {start} => {mask:018b}");
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
					let val = self.mmu.read_word(cur);
					self.write_dr(reg, Size::Word, val as u32);
					cur += 2;
				}
			}			
			let mask = mask >> 8;
			for reg in 0..8 {
				// Addr
				if is_bit_set(mask, reg) {
					let val = self.mmu.read_word(cur);
					self.write_ar(reg, val as u32);
					cur += 2;
				}
			}
		} else {
			// Register to Memory
			trace!("MOVEM.w {mask:#X} => {ea:?}: {start:#X}");
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

			for reg in 0..8 {
				// D
				if is_bit_set(mask, reg) {
					let val = self.read_dr(reg);
					self.mmu.write_word(cur, val as u16);
					cur += 2;
				}
			}
			let mask = mask >> 8;
			for reg in 0..8 {
				// A
				if is_bit_set(mask, reg) {
					let val = self.read_ar(reg);
					self.mmu.write_word(cur, val as u16);
					cur += 2;
				}
			}
		}
	}
}