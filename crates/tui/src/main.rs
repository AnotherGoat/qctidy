mod app;
mod picker;
mod ui;

use std::io::{self, Write};

use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::app::App;
use crate::ui::render;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new();

    while !app.should_quit {
        terminal.draw(|frame| render(frame, &mut app))?;

        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event::read()?
        {
            app.handle_key(code);
        }

        app.tick();
    }

    let restore_result = (|| -> io::Result<()> {
        disable_raw_mode()?;
        terminal.backend_mut().execute(LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    })();

    if let Err(e) = restore_result {
        let _result = writeln!(io::stderr(), "Error restoring terminal: {e}");
    }

    Ok(())
}
