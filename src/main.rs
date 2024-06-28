mod app;
mod cell;
mod errors;
mod playground;
mod tui;
mod worm;

use crate::app::App;

fn main() -> errors::Result {
    errors::install_hooks()?;

    let mut terminal = tui::init()?;
    App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
