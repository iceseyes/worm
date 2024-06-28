use crate::cell::{Cell, Direction, Point};
use itertools::Itertools;
use rand::Rng;

#[derive(Debug)]
pub struct Worm(Vec<Cell>);

impl Worm {
    pub fn new(rng: &mut (impl Rng + ?Sized)) -> Self {
        Self(vec![Cell::at_random(rng)])
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn head(&self) -> &Point {
        self.0.first().unwrap().point()
    }

    pub fn eat(&mut self, p: &Point) -> bool {
        let head = self.0.first().unwrap();
        if !head.is_next_position(p) {
            return false;
        }

        let candidate = Cell::new(p.clone(), head.direction());
        self.0.insert(0, candidate);

        true
    }

    pub fn turn(&mut self, d: &Direction) {
        let mut head = self.0.remove(0);
        head.turn(d);
        self.0.insert(0, head);
    }

    pub fn step(&mut self) {
        let mut last_direction = self.0.first().unwrap().direction();
        self.0.iter_mut().for_each(|cell| {
            cell.step();
            let current_direction = cell.direction();
            if last_direction != current_direction {
                cell.turn(&last_direction);
                last_direction = current_direction;
            }
        });
    }

    pub fn points(&self) -> Vec<&Point> {
        self.0.iter().map(|c| c.point()).collect()
    }

    pub fn is_crashed(&self) -> bool {
        self.0.iter().all_unique()
    }
}

impl Default for Worm {
    fn default() -> Self {
        Self(vec![Cell::default()])
    }
}

#[cfg(test)]
mod test {
    use crate::cell::{Cell, Direction, Point};
    use crate::worm::Worm;

    #[test]
    fn test_walk() {
        let mut worm = Worm::default();
        assert!(!worm.eat(&Point::new(128, 128)));
        assert_eq!(worm.0.len(), 1);

        assert!(!worm.eat(&Point::new(1, 0)));
        assert_eq!(worm.0.len(), 1);

        assert!(worm.eat(&Point::new(0, 1)));
        assert_eq!(
            worm.0,
            [
                Cell::new((0, 1).into(), Direction::Up),
                Cell::new((0, 0).into(), Direction::Up),
            ]
        );

        worm.step();
        assert_eq!(
            worm.0,
            [
                Cell::new((0, 2).into(), Direction::Up),
                Cell::new((0, 1).into(), Direction::Up),
            ]
        );

        worm.turn(&Direction::Right);
        assert_eq!(
            worm.0,
            [
                Cell::new((0, 2).into(), Direction::Right),
                Cell::new((0, 1).into(), Direction::Up),
            ]
        );

        worm.step();
        assert_eq!(
            worm.0,
            [
                Cell::new((1, 2).into(), Direction::Right),
                Cell::new((0, 2).into(), Direction::Right),
            ]
        );

        assert!(worm.eat(&Point::new(2, 2)));
        assert_eq!(
            worm.0,
            [
                Cell::new((2, 2).into(), Direction::Right),
                Cell::new((1, 2).into(), Direction::Right),
                Cell::new((0, 2).into(), Direction::Right),
            ]
        );

        worm.step();
        assert_eq!(
            worm.0,
            [
                Cell::new((3, 2).into(), Direction::Right),
                Cell::new((2, 2).into(), Direction::Right),
                Cell::new((1, 2).into(), Direction::Right),
            ]
        );

        worm.turn(&Direction::Down);
        assert_eq!(
            worm.0,
            [
                Cell::new((3, 2).into(), Direction::Down),
                Cell::new((2, 2).into(), Direction::Right),
                Cell::new((1, 2).into(), Direction::Right),
            ]
        );

        worm.step();
        assert_eq!(
            worm.0,
            [
                Cell::new((3, 1).into(), Direction::Down),
                Cell::new((3, 2).into(), Direction::Down),
                Cell::new((2, 2).into(), Direction::Right),
            ]
        );

        worm.step();
        assert_eq!(
            worm.0,
            [
                Cell::new((3, 0).into(), Direction::Down),
                Cell::new((3, 1).into(), Direction::Down),
                Cell::new((3, 2).into(), Direction::Down),
            ]
        );
    }
}
