use crate::cell::{Direction, Point};
use crate::worm::Worm;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use ratatui::prelude::{Color, Marker};
use ratatui::widgets::canvas::{Canvas, Rectangle};
use ratatui::widgets::{Block, Widget};
use std::default::Default;

#[derive(Debug)]
pub struct Playground {
    rng: ThreadRng,
    worm: Worm,
    food: Point,
}

impl Playground {
    const PIXEL_FACTOR: f64 = 100.0;

    pub fn tick(&mut self) {
        if self.worm.eat(&self.food) {
            self.food = Point::at_random(&mut self.rng)
        }

        self.worm.step();
    }

    pub fn alive(&self) -> bool {
        !self.worm.is_crashed()
    }

    pub fn move_left(&mut self) {
        self.worm.turn(&Direction::Left);
    }

    pub fn move_right(&mut self) {
        self.worm.turn(&Direction::Right);
    }

    pub fn move_down(&mut self) {
        self.worm.turn(&Direction::Down);
    }

    pub fn move_up(&mut self) {
        self.worm.turn(&Direction::Up);
    }

    fn get_x(p: &Point) -> f64 {
        (p.x as f64 - (u8::MAX as f64) / 2.0) * Self::PIXEL_FACTOR
    }

    fn get_y(p: &Point) -> f64 {
        (p.y as f64 - (u8::MAX as f64) / 2.0) * Self::PIXEL_FACTOR
    }

    pub fn canvas<'a>(&'a self) -> impl Widget + 'a {
        let head = self.worm.head();
        let title = format!("Worm: {} ({:03}, {:03})", self.worm.size(), head.x, head.y);
        Canvas::default()
            .block(Block::bordered().title(title))
            .marker(Marker::Braille)
            .x_bounds([
                Self::get_x(&(0u8, 0u8).into()),
                Self::get_x(&(u8::MAX, u8::MAX).into()),
            ])
            .y_bounds([
                Self::get_y(&(0u8, 0u8).into()),
                Self::get_y(&(u8::MAX, u8::MAX).into()),
            ])
            .paint(|ctx| {
                ctx.draw(&Self::rectangle(&self.food, Default::default()));

                self.worm
                    .points()
                    .iter()
                    .for_each(|&p| ctx.draw(&Self::rectangle(p, Color::Red)));
            })
    }

    fn rectangle(p: &Point, color: Color) -> Rectangle {
        Rectangle {
            x: Self::get_x(p),
            y: Self::get_y(p),
            width: Self::PIXEL_FACTOR,
            height: Self::PIXEL_FACTOR,
            color,
        }
    }
}

impl Default for Playground {
    fn default() -> Self {
        let mut rng = thread_rng();
        let worm = Worm::new(&mut rng);
        let food = Point::at_random(&mut rng);
        Self { rng, worm, food }
    }
}
