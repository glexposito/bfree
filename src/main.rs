use std::io;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Frame, Terminal, backend::CrosstermBackend, text::Text, widgets::Paragraph};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    loop {
        terminal
            .draw(draw)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello bfree command!");
    let widget = Paragraph::new(text);
    frame.render_widget(widget, frame.area());
}
