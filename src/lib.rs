use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;

use app::App;
use eyre::Result;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::ui;

pub mod app;

#[allow(unreachable_code)]
pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let app = app.borrow();

        terminal.draw(|rect| ui::draw(rect, &app))?;
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}