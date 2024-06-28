use rand::Rng;
use std::ops::Rem;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Cell(Point, Direction);

impl Cell {
    pub fn new(p: Point, d: Direction) -> Self {
        Self(p, d)
    }

    pub fn at_random(rng: &mut (impl Rng + ?Sized)) -> Self {
        Self(Point::at_random(rng), Direction::at_random(rng))
    }

    pub fn is_next_to(&self, other: &Self) -> bool {
        self.0.is_next_to(&other.0)
    }

    pub fn is_linked(&self, other: &Self) -> bool {
        self.is_next_to(other) && self.1 == other.1
    }

    pub fn is_next_position(&self, d: &Point) -> bool {
        let mut p = self.0.clone();
        p.step_to(&self.1);
        p == *d
    }

    pub fn direction(&self) -> Direction {
        self.1.clone()
    }

    pub fn point(&self) -> &Point {
        &self.0
    }

    pub fn turn(&mut self, d: &Direction) {
        self.1 = d.clone();
    }

    pub fn step(&mut self) {
        self.0.step_to(&self.1);
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn at_random(rng: &mut (impl Rng + ?Sized)) -> Self {
        let d: u8 = rng.gen();
        match d.rem(4) {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => Direction::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn at_random(rng: &mut (impl Rng + ?Sized)) -> Self {
        Self::new(rng.gen(), rng.gen())
    }

    pub fn step_to(&mut self, dir: &Direction) {
        let Self { x, y } = self;
        match dir {
            Direction::Up => *y = y.wrapping_add(1u8),
            Direction::Right => *x = x.wrapping_add(1u8),
            Direction::Down => *y = y.wrapping_sub(1u8),
            Direction::Left => *x = x.wrapping_sub(1u8),
        };
    }

    pub fn is_next_to(&self, other: &Self) -> bool {
        ((self.x.saturating_sub(1) <= other.x
            && self.x.saturating_add(1) >= other.x
            && self.y == other.y)
            || (self.y.saturating_sub(1) <= other.y
                && self.y.saturating_add(1) >= other.y
                && self.x == other.x)
            || ((self.x.wrapping_sub(1) == other.x || self.x.wrapping_add(1) == other.x)
                && self.y == other.y)
            || ((self.y.wrapping_sub(1) == other.y || self.y.wrapping_add(1) == other.y)
                && self.x == other.x))
            && self != other
    }
}

impl From<Point> for (u8, u8) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

impl From<(u8, u8)> for Point {
    fn from(value: (u8, u8)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cell::{Cell, Direction, Point};
    use rand::rngs::mock::StepRng;

    #[test]
    fn default_point() {
        assert_eq!(Point::default(), Point::new(0, 0));
        assert_eq!(Point::default(), (0, 0).into());
    }

    #[test]
    fn point_into_pair() {
        let p: (u8, u8) = Point::default().into();
        assert_eq!(p, (0, 0));

        let p: (u8, u8) = Point::new(100, 34).into();
        assert_eq!(p, (100, 34));

        let p: (u8, u8) = Point::new(34, 45).into();
        assert_eq!(p, (34, 45));
    }

    #[test]
    fn test_point_step_to() {
        let mut p = Point::default();

        p.step_to(&Direction::Up);
        assert_eq!(p, (0, 1).into());

        p.step_to(&Direction::Down);
        assert_eq!(p, (0, 0).into());

        p.step_to(&Direction::Right);
        assert_eq!(p, (1, 0).into());

        p.step_to(&Direction::Left);
        assert_eq!(p, (0, 0).into());

        p.step_to(&Direction::Left);
        assert_eq!(p, (255, 0).into());

        p.step_to(&Direction::Down);
        assert_eq!(p, (255, 255).into());

        p.step_to(&Direction::Up);
        assert_eq!(p, (255, 0).into());

        p.step_to(&Direction::Right);
        assert_eq!(p, (0, 0).into());
    }

    #[test]
    fn point_at_random() {
        let mut rng = StepRng::new(15, 1);
        let p = Point::at_random(&mut rng);
        assert_eq!(p, (15, 16).into());

        let p = Point::at_random(&mut rng);
        assert_eq!(p, (17, 18).into());
    }

    #[test]
    fn default_direction() {
        assert_eq!(Direction::default(), Direction::Up);
    }

    #[test]
    fn default_at_random() {
        let mut rng = StepRng::new(0, 1);
        assert_eq!(Direction::at_random(&mut rng), Direction::Up);
        assert_eq!(Direction::at_random(&mut rng), Direction::Right);
        assert_eq!(Direction::at_random(&mut rng), Direction::Down);
        assert_eq!(Direction::at_random(&mut rng), Direction::Left);
        assert_eq!(Direction::at_random(&mut rng), Direction::Up);
        assert_eq!(Direction::at_random(&mut rng), Direction::Right);
    }

    #[test]
    fn default_cell() {
        let c = Cell::default();
        let Cell(p, d) = c;
        assert_eq!(p, Point::default());
        assert_eq!(d, Direction::Up)
    }

    #[test]
    fn cell_at_random() {
        let mut rng = StepRng::new(15, 1);
        let Cell(p, d) = Cell::at_random(&mut rng);
        assert_eq!(p, (15, 16).into());
        assert_eq!(d, Direction::Right);
    }

    #[test]
    fn test_cell_is_near() {
        let c1 = Cell(Point::new(128, 128), Direction::Up);

        let c2 = Cell(Point::new(129, 127), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(129, 128), Direction::Up);
        assert!(c1.is_next_to(&c2));

        let c2 = Cell(Point::new(129, 129), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(128, 127), Direction::Up);
        assert!(c1.is_next_to(&c2));

        let c2 = Cell(Point::new(128, 128), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(128, 129), Direction::Up);
        assert!(c1.is_next_to(&c2));

        let c2 = Cell(Point::new(127, 129), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(127, 128), Direction::Up);
        assert!(c1.is_next_to(&c2));

        let c2 = Cell(Point::new(127, 127), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(126, 127), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(126, 128), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(130, 129), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(129, 128), Direction::Down);
        assert!(c1.is_next_to(&c2));

        let c2 = Cell(Point::new(129, 128), Direction::Left);
        assert!(c1.is_next_to(&c2));

        let c2 = Cell(Point::new(129, 128), Direction::Right);
        assert!(c1.is_next_to(&c2));
    }

    #[test]
    fn test_cell_is_near_on_the_edge() {
        let c1 = Cell(Point::new(0, 0), Direction::Up);
        let c2 = Cell(Point::new(255, 255), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(255, 0), Direction::Up);
        assert!(c1.is_next_to(&c2));

        let c2 = Cell(Point::new(255, 1), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(0, 255), Direction::Up);
        assert!(c1.is_next_to(&c2));

        let c2 = Cell(Point::new(0, 0), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(0, 1), Direction::Up);
        assert!(c1.is_next_to(&c2));

        let c1 = Cell(Point::new(255, 255), Direction::Up);
        let c2 = Cell(Point::new(0, 0), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(0, 255), Direction::Up);
        assert!(c1.is_next_to(&c2));

        let c2 = Cell(Point::new(0, 254), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(255, 0), Direction::Up);
        assert!(c1.is_next_to(&c2));

        let c2 = Cell(Point::new(255, 255), Direction::Up);
        assert!(!c1.is_next_to(&c2));

        let c2 = Cell(Point::new(255, 254), Direction::Up);
        assert!(c1.is_next_to(&c2));
    }

    #[test]
    fn test_is_linked() {
        let c1 = Cell(Point::new(128, 128), Direction::Up);

        let c2 = Cell(Point::new(129, 127), Direction::Up);
        assert!(!c1.is_linked(&c2));

        let c2 = Cell(Point::new(129, 128), Direction::Up);
        assert!(c1.is_linked(&c2));

        let c2 = Cell(Point::new(129, 128), Direction::Down);
        assert!(!c1.is_linked(&c2));
    }
}
