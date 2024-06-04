use ratatui::{
    prelude::*,
    widgets::{Paragraph, Widget},
};

pub struct Stackview<'a> {
    ram: &'a [u8],
    sp: usize,
}

impl<'a> Stackview<'a> {
    pub fn new(ram: &'a [u8], sp: usize) -> Self {
        Self { ram, sp }
    }
}

impl<'a> Widget for Stackview<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let height = area.height;
        let mut string = String::with_capacity(256);
        let start_idx = self.sp.wrapping_sub((height as usize * 4) / 2);
        let end_idx = self.sp.wrapping_add((height as usize * 4) / 2);
        for i in (start_idx..=end_idx).step_by(4) {
            string += &format_line(self.ram, i & 0xFFFFFF, self.sp);
        }

        Paragraph::new(string).render(area, buf);
    }
}

fn format_line(ram: &[u8], addr: usize, sp: usize) -> String {
    if addr == (sp & 0xFFFFFF) || addr == ((sp + 2) & 0xFFFFFF) {
        format!(
            "=>{addr:#08X}: {:02X} {:02X} {:02X} {:02X}\n",
            ram[addr],
            ram[addr + 1],
            ram[addr + 2],
            ram[addr + 3]
        )
    } else {
        format!(
            "  {addr:#08X}: {:02X} {:02X} {:02X} {:02X}\n",
            ram[addr],
            ram[addr + 1],
            ram[addr + 2],
            ram[addr + 3]
        )
    }
}
