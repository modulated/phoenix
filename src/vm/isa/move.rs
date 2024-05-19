use crate::{
    types::{AddressingMode, Size, Value},
    util::{get_bits, get_size, sign_extend_16_to_32, SizeCoding},
    vm::cpu::Cpu,
};

impl<'a> Cpu<'a> {
    pub(super) fn move_family(&mut self, inst: u16) {
        if get_bits(inst, 6, 3) == 0b001 {
            return self.movea(inst);
        }
        self.r#move(inst);
    }

    fn movea(&mut self, inst: u16) {
        let size = (0b0011_0000_0000_0000 & inst) >> 12;
        let size = match size {
            0b01 => Size::Word,
            0b10 => Size::Long,
            _ => unreachable!(),
        };
        let dst = get_bits(inst, 9, 3);
        let ea = AddressingMode::from(inst);
        let val = self.read_ea(ea, size);
        println!("MOVEA A{dst} <= {ea:?}: {val}");
        match val {
            Value::Word(v) => self.write_ar(dst.try_into().unwrap(), sign_extend_16_to_32(v)),
            Value::Long(v) => self.write_ar(dst.try_into().unwrap(), v),
            Value::Byte(_) => unreachable!(),
        }
    }

    fn r#move(&mut self, inst: u16) {
        let size = get_size(inst, 12, SizeCoding::Purple);
        let dst = AddressingMode::from((get_bits(inst, 6, 3) << 3) + get_bits(inst, 9, 3));
        let src = AddressingMode::from(inst);
        let val = self.read_ea(src, size);
        println!("MOVE {dst:?} <= {src:?}: {val}");
        self.write_ea(dst, size, val);

        // TODO: status flags
    }
}
