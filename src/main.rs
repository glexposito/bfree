use std::{collections::HashMap, fs, io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

struct App {
    mem_total: u64,
    mem_free: u64,
    mem_available: u64,
    mem_buffers: u64,
    mem_cached: u64,
    swap_total: u64,
    swap_free: u64,
    mem_shared: u64,
}

impl App {
    fn new() -> Self {
        Self {
            mem_total: 0,
            mem_free: 0,
            mem_available: 0,
            mem_buffers: 0,
            mem_cached: 0,
            swap_total: 0,
            swap_free: 0,
            mem_shared: 0,
        }
    }

    fn update(&mut self) -> io::Result<()> {
        let content = fs::read_to_string("/proc/meminfo")?;
        let mut meminfo = HashMap::new();

        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value_str = parts[1].trim();
                let value_parts: Vec<&str> = value_str.split_whitespace().collect();
                if let Some(numeric_str) = value_parts.first() {
                    if let Ok(value) = numeric_str.parse::<u64>() {
                        meminfo.insert(key, value);
                    }
                }
            }
        }

        self.mem_total = meminfo.get("MemTotal").cloned().unwrap_or(0);
        self.mem_free = meminfo.get("MemFree").cloned().unwrap_or(0);
        self.mem_available = meminfo.get("MemAvailable").cloned().unwrap_or(0);
        self.mem_buffers = meminfo.get("Buffers").cloned().unwrap_or(0);
        self.mem_cached = meminfo.get("Cached").cloned().unwrap_or(0);
        self.mem_shared = meminfo.get("Shmem").cloned().unwrap_or(0);
        self.swap_total = meminfo.get("SwapTotal").cloned().unwrap_or(0);
        self.swap_free = meminfo.get("SwapFree").cloned().unwrap_or(0);

        Ok(())
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        app.update()?;
        terminal
            .draw(|f| ui(f, app))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let main_block = Block::default().title("bfree").borders(Borders::ALL);
    let main_area = main_block.inner(frame.area());
    frame.render_widget(main_block, frame.area());

    let mem_block = Block::default().title("Memory").borders(Borders::NONE);
    let mem_area = mem_block.inner(main_area);
    frame.render_widget(mem_block, main_area);

    let mem_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1), // Total
                Constraint::Length(2), // Used
                Constraint::Length(2), // Available
                Constraint::Length(2), // Cached
                Constraint::Length(2), // Free
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(mem_area);

    let to_gib = |value_kb| value_kb as f64 / 1024.0 / 1024.0;

    // Total
    let total_mem_gib = to_gib(app.mem_total);
    let total_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(mem_chunks[0]);

    let total_label = Paragraph::new("Total:");
    frame.render_widget(total_label, total_chunks[0]);

    let total_value =
        Paragraph::new(format!("{:.2} GiB", total_mem_gib)).alignment(Alignment::Right);
    frame.render_widget(total_value, total_chunks[1]);

    // Categories
    let used_val = app.mem_total - app.mem_free - app.mem_buffers - app.mem_cached;
    let categories = [
        ("Used", used_val, Color::Red),
        ("Available", app.mem_available, Color::Yellow),
        ("Cached", app.mem_cached + app.mem_buffers, Color::Green),
        ("Free", app.mem_free, Color::Blue),
    ];

    for (i, (name, value_kb, color)) in categories.iter().enumerate() {
        let percent = if app.mem_total > 0 {
            (*value_kb * 100) / app.mem_total
        } else {
            0
        };
        let value_gib = to_gib(*value_kb);

        let row_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(12),
                    Constraint::Min(10),
                    Constraint::Length(12),
                ]
                .as_ref(),
            )
            .split(mem_chunks[i + 1]);

        // Label
        let label = Paragraph::new(vec![
            Line::from(Span::raw(name.to_string())),
            Line::from(Span::raw(format!("{}%", percent))),
        ]);
        frame.render_widget(label, row_chunks[0]);

        // Bar
        let bar_width = row_chunks[1].width as usize;
        let bar_char_count = (value_gib / total_mem_gib * bar_width as f64).ceil() as usize;
        let bar_text = "█".repeat(bar_char_count);
        let bar = Paragraph::new(bar_text)
            .style(Style::default().fg(*color))
            .alignment(Alignment::Left);
        frame.render_widget(bar, row_chunks[1]);

        // Value
        let value_text =
            Paragraph::new(format!("{:.2} GiB", value_gib)).alignment(Alignment::Right);
        frame.render_widget(value_text, row_chunks[2]);
    }
}
