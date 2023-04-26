use std::cell::RefCell;
use std::rc::Rc;

use eyre::Result;
use terminal_pomodoro::app::App;
use terminal_pomodoro::start_ui;

fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new()));
    start_ui(app)?;
    Ok(())
}
