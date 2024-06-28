use crate::playground::Playground;
use crate::{errors, tui};
use color_eyre::eyre::Context;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    playground: Playground,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> errors::Result {
        let tick_rate = Duration::from_millis(20);
        let mut last_tick = Instant::now();
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            if event::poll(Duration::from_millis(10))? {
                self.handle_events().wrap_err("handle events failed")?;
            }

            if last_tick.elapsed() >= tick_rate {
                self.playground.tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self.playground.canvas(), frame.size());
    }

    fn handle_events(&mut self) -> errors::Result {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),

            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> errors::Result {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => {
                self.playground.move_left();
                Ok(())
            }
            KeyCode::Right => {
                self.playground.move_right();
                Ok(())
            }
            KeyCode::Up => {
                self.playground.move_up();
                Ok(())
            }
            KeyCode::Down => {
                self.playground.move_down();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn exit(&mut self) -> errors::Result {
        self.exit = true;
        Ok(())
    }
}
