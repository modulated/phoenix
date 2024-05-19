use ratatui::{
    prelude::*,
    widgets::{Paragraph, Widget},
};

pub struct Memview<'a> {
    ram: &'a [u8],
    pc: usize,
}

impl<'a> Memview<'a> {
    pub fn new(ram: &'a [u8], pc: usize) -> Self {
        Self { ram, pc }
    }
}

impl<'a> Widget for Memview<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut string = String::with_capacity(256);
        let start_idx = self.pc.saturating_sub(area.height as usize);
        let end_idx = start_idx + (area.height * 2) as usize;
        for i in (start_idx..=end_idx).step_by(2) {
            string += &format_line(self.ram, i, i == self.pc);
        }

        Paragraph::new(string).render(area, buf);
    }
}

fn format_line(ram: &[u8], idx: usize, highlighted: bool) -> String {
    if highlighted {
        format!(
            "=>{idx:#08X} \t{:#010b}{:08b} \t {:#04X}{:02X}\n",
            ram[idx],
            ram[idx + 1],
            ram[idx],
            ram[idx + 1]
        )
    } else {
        format!(
            "  {idx:#08X} \t{:#010b}{:08b} \t {:#04X}{:02X}\n",
            ram[idx],
            ram[idx + 1],
            ram[idx],
            ram[idx + 1]
        )
    }
}
