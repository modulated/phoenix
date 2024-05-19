use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use memview::Memview;
use phoenix::{StatusRegister, VM};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Row, Table},
};
use std::io::{stdout, Result};

mod memview;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut vm = VM::new();
    let mut mode = Mode::Paused;

    let rom = std::fs::read("roms/memcheck.bin").unwrap();
    vm.load(&rom);

    loop {
        terminal.draw(|frame| {
            let reg_block = Block::default().borders(Borders::all()).title("Registers");
            let inst_block = Block::default()
                .borders(Borders::all())
                .title("Instructions");

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(Constraint::from_percentages(vec![50, 50]))
                .split(frame.size());
            let sub_right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(Constraint::from_percentages(vec![50, 50]))
                .split(layout[1]);

            frame.render_widget(
                Block::default().borders(Borders::all()).title("Memory"),
                layout[0],
            );
            frame.render_widget(reg_block.clone(), sub_right[0]);
            frame.render_widget(inst_block.clone(), sub_right[1]);

            frame.render_widget(create_reg_widget(&vm), reg_block.inner(sub_right[0]));

            frame.render_widget(
                Memview::new(vm.cpu.mmu.get_slice(), vm.read_pc() as usize),
                inst_block.inner(sub_right[1]),
            );
        })?;

        match mode {
            Mode::Step => {
                vm.step();
                mode = Mode::Paused;
            }
            Mode::Running => vm.step(),
            Mode::SetPC => {}
            Mode::Paused => {}
        }

        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('p') => mode = Mode::SetPC,
                            KeyCode::Char(' ') => mode = Mode::Step,
                            KeyCode::Enter => mode = Mode::Running,
                            _ => {}
                        }
                    }
                }
                Event::FocusLost => mode = Mode::Paused,
                _ => {}
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

enum Mode {
    Paused,
    Step,
    Running,
    SetPC,
}

fn create_reg_widget(vm: &VM) -> impl Widget {
    Table::new(
        vec![
            Row::new(vec![
                "D0".to_string(),
                format!("{:#010X}", vm.read_dr()[0]),
                "A0".to_string(),
                format!("{:#010X}", vm.read_ar()[0]),
            ]),
            Row::new(vec![
                "D1".to_string(),
                format!("{:#010X}", vm.read_dr()[1]),
                "A1".to_string(),
                format!("{:#010X}", vm.read_ar()[1]),
            ]),
            Row::new(vec![
                "D2".to_string(),
                format!("{:#010X}", vm.read_dr()[2]),
                "A2".to_string(),
                format!("{:#010X}", vm.read_ar()[2]),
            ]),
            Row::new(vec![
                "D3".to_string(),
                format!("{:#010X}", vm.read_dr()[3]),
                "A3".to_string(),
                format!("{:#010X}", vm.read_ar()[3]),
            ]),
            Row::new(vec![
                "D4".to_string(),
                format!("{:#010X}", vm.read_dr()[4]),
                "A4".to_string(),
                format!("{:#010X}", vm.read_ar()[4]),
            ]),
            Row::new(vec![
                "D5".to_string(),
                format!("{:#010X}", vm.read_dr()[5]),
                "A5".to_string(),
                format!("{:#010X}", vm.read_ar()[5]),
            ]),
            Row::new(vec![
                "D6".to_string(),
                format!("{:#010X}", vm.read_dr()[6]),
                "A6".to_string(),
                format!("{:#010X}", vm.read_ar()[6]),
            ]),
            Row::new(vec![
                "D7".to_string(),
                format!("{:#010X}", vm.read_dr()[7]),
                "".to_string(),
                "".to_string(),
            ]),
            Row::new(vec![
                "SSP".to_string(),
                format!("{:#010X}", vm.read_ssp()),
                "USP".to_string(),
                format!("{:#010X}", vm.read_usp()),
            ]),
            Row::new(vec!["".to_string(), "".to_string(), "".to_string()]),
            Row::new(vec![
                "PC".to_string(),
                format!("{:#010X}", vm.read_pc()),
                "SR".to_string(),
                format!(
                    "X{} N{} Z{} V{} C{}",
                    vm.read_sr(StatusRegister::X) as u8,
                    vm.read_sr(StatusRegister::N) as u8,
                    vm.read_sr(StatusRegister::Z) as u8,
                    vm.read_sr(StatusRegister::V) as u8,
                    vm.read_sr(StatusRegister::C) as u8
                ),
            ]),
            Row::new(vec![
                "".to_string(),
                "Time:".to_string(),
                format!("{} nanos", vm.inst_time).to_string(),
                format!("{} MHz", 1000000 / vm.inst_time).to_string(),
            ]),
        ],
        vec![
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(40),
        ],
    )
}
