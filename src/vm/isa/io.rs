use log::info;

use crate::vm::cpu::Cpu;

impl<'a> Cpu<'a> {
    pub(crate) fn console_trap(&mut self) {
        let task = self.read_dr(0);
        match task {
            0 => self.println_string(),
            1 => self.print_string(),
            2 => self.read_string(),
            3 => self.display_signed_int(),
            4 => self.read_num(),
            5 => self.read_char(),
            6 => self.print_char(),
            7 => self.pending_char(),
            8 => self.get_time(),
            9 => self.io_halt(),
            10 => self.println_string_terminated(),
            11 => unimplemented!("Cursor positioning"),
            12 => unimplemented!("Key echo"),
            13 => self.println_string_terminated(),
            14 => self.print_string_terminated(),
            15 => self.print_unsigned_int(),
            _ => unimplemented!("Task {task} unknown"),
        }
    }
    pub(crate) fn print_string(&mut self) {
        todo!()
    }

    pub(crate) fn println_string(&mut self) {
        todo!()
    }

    pub(crate) fn read_string(&mut self) {
        todo!()
    }

    pub(crate) fn display_signed_int(&mut self) {
        let num = self.read_dr(1);
        info!("{num}");
    }

    pub(crate) fn read_num(&mut self) {
        todo!()
    }

    pub(crate) fn read_char(&mut self) {
        todo!()
    }

    pub(crate) fn print_char(&mut self) {
        todo!()
    }

    pub(crate) fn pending_char(&mut self) {
        todo!()
    }

    pub(crate) fn get_time(&mut self) {
        todo!()
    }

    pub(crate) fn io_halt(&mut self) {
        self.halt()
    }

    pub(crate) fn println_string_terminated(&mut self) {
        todo!()
    }

    pub(crate) fn print_string_terminated(&mut self) {
        let mut addr = self.read_ar(1);
        let mut byte = 0xFF;
        let mut string = vec![];
        while byte != 0x00 {
            byte = self.mmu.read_byte(addr);
            string.push(byte);
            addr += 1;
        }
        let string = String::from_utf8(string).expect("Could not read bytes");
        info!("{string}");
    }

    pub(crate) fn print_unsigned_int(&mut self) {
        todo!()
    }
}
