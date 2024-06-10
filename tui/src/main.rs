use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use log::info;
use memview::Memview;
use phoenix::{Args, VM};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Row, Table},
};
use simplelog::{ConfigBuilder, WriteLogger};
use std::{
    cmp::max,
    fs,
    io::{stdout, Result, Stdout, Write},
    sync::{Arc, Mutex},
};
mod memview;
mod stackview;
use stackview::Stackview;

#[derive(Debug, Clone)]
struct Log {
    pub log: Arc<Mutex<Vec<String>>>,
}

impl Write for Log {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let s = match std::str::from_utf8(buf) {
            Ok(s) => s,
            Err(e) => panic!("Cannot parse to string: {e}"),
        };
        let mut l = self.log.lock().unwrap();
        l.push(s.to_string());
        Ok(s.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

struct Cleanup {}

impl Drop for Cleanup {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }
}

fn main() -> Result<()> {
    let _ = Cleanup {};
    let args = Args::parse();
    let mut terminal = init_term()?;
    let mut mode = Mode::Paused;
    let conf = ConfigBuilder::new()
        .set_time_level(log::LevelFilter::Off)
        .set_thread_level(log::LevelFilter::Off)
        .set_location_level(log::LevelFilter::Off)
        .set_target_level(log::LevelFilter::Off)
        .set_max_level(log::LevelFilter::Off)
        .build();
    let log = Log {
        log: Arc::new(Mutex::new(vec![])),
    };
    let _ = WriteLogger::init(log::LevelFilter::Trace, conf, log.clone());
    info!("Starting VM");
    let mut vm = VM::new();
    if let Ok(pc_addr) = u32::from_str_radix(&args.program_counter, 16) {
        vm.set_pc(pc_addr);
        info!("PC set to {pc_addr:#X}");
    }
    if let Some(usp) = args.user_stack_pointer {
        let usp = u32::from_str_radix(&usp, 16).expect("Could not parse USP");
        vm.cpu.write_usp(usp);
        info!("USP set to {usp:#X}");
    }
    if let Some(ssp) = args.system_stack_pointer {
        let ssp = u32::from_str_radix(&ssp, 16).expect("Could not parse SSP");
        vm.set_sp(ssp);
        info!("SSP set to {ssp:#X}");
    }
    info!("Loading program");
    let rom = fs::read(&args.file)
        .unwrap_or_else(|_| panic!("Could not open provided file {}", args.file));
    vm.load(&rom);
    info!("{} bytes loaded to RAM", rom.len());

    loop {
        terminal.draw(|frame| {
            let layout_vert = Layout::default()
                .direction(Direction::Vertical)
                .constraints(Constraint::from_percentages(vec![80, 20]))
                .split(frame.size());
            let reg_block = Block::default().borders(Borders::all()).title("Registers");
            let inst_block = Block::default()
                .borders(Borders::all())
                .title("Instructions");
            let stack_block = Block::default().borders(Borders::all()).title("Stack");

            // let layout_hor = Layout::default()
            //     .direction(Direction::Horizontal)
            //     .constraints(Constraint::from_percentages(vec![50, 50]))
            //     .split(layout_vert[0]);
            let sub_right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(Constraint::from_percentages(vec![50, 50]))
                .split(layout_vert[0]);

            let memory_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(Constraint::from_percentages(vec![50, 50]))
                .split(sub_right[1]);

            frame.render_widget(create_log_widget(&log), layout_vert[1]);

            // frame.render_widget(
            //     Block::default().borders(Borders::all()).title("Memory"),
            //     layout_hor[0],
            // );
            frame.render_widget(reg_block.clone(), sub_right[0]);
            frame.render_widget(inst_block.clone(), memory_layout[0]);
            frame.render_widget(stack_block.clone(), memory_layout[1]);

            frame.render_widget(create_reg_widget(&vm), reg_block.inner(sub_right[0]));

            frame.render_widget(
                Memview::new(vm.cpu.mmu.get_slice(), vm.read_pc() as usize),
                inst_block.inner(memory_layout[0]),
            );

            frame.render_widget(
                Stackview::new(vm.cpu.mmu.get_slice(), vm.cpu.read_ar(7) as usize),
                stack_block.inner(memory_layout[1]),
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

fn init_term() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let default_panic = std::panic::take_hook();
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    std::panic::set_hook(Box::new(move |p| {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
        default_panic(p);
    }));
    Ok(terminal)
}

fn create_log_widget(log: &Log) -> impl Widget {
    let l = log.log.lock().unwrap();
    let l = l.join("");
    let lines = l.lines();
    let len = lines.clone().collect::<Vec<&str>>().len();
    let mut string = String::new();
    for s in lines.skip(max(len as i32 - 9i32, 0) as usize).take(9) {
        string.push_str(&format!("{}\n", s));
    }
    Paragraph::new(string)
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
                "A7".to_string(),
                format!("{:#010X}", vm.cpu.read_ar(7)),
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
                format!("{:#018b}", vm.cpu.read_sr()),
            ]),
            Row::new(vec![
                "".to_string(),
                "Time:".to_string(),
                format!("{} ns", vm.inst_time).to_string(),
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
