use crate::point::Point;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(width: u16, height: u16) -> Snake {
        let start = Point {
            x: width / 2,
            y: height / 2,
        };

        Snake {
            body: vec![start],
            direction: Direction::Right,
        }
    }

    pub fn get_head(&self) -> &Point {
        self.body.first().unwrap()
    }
}
