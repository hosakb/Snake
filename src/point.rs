#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }
}

impl From<Point> for crossterm::cursor::MoveTo{
    fn from(p: Point) -> Self {
       Self(p.x, p.y)
    }
}
