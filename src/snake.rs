use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};

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
        Snake {
            body: vec![Point {
                x: width / 2,
                y: height / 2,
            }],
            direction: Direction::Right,
        }
    }

    pub fn get_head(&mut self) -> &mut Point {
        self.body.first_mut().unwrap()
    }

    pub fn get_direc(&mut self) {
        if poll(Duration::from_millis(500)).expect("Error polling key press") {
            if let Event::Key(event) = read().expect("Error reading keys") {
                self.direction = match event.code {
                    KeyCode::Down => Direction::Down,
                    KeyCode::Up => Direction::Up,
                    KeyCode::Left => Direction::Left,
                    KeyCode::Right => Direction::Right,
                    _ => self.direction,
                }
            }
        }
    }

    pub fn change_direc(&mut self) {
        match self.direction {
            Direction::Down => self.get_head().y -= 1,
            Direction::Up => self.get_head().y += 1,
            Direction::Left => self.get_head().x -= 1,
            Direction::Right => self.get_head().x += 1,
        }
    }

    pub fn eat(&mut self, food: Point) -> bool {
        let head = *self.get_head();

        if food == head {
            self.body.insert(0, food);
            true
        } else {
            self.body.insert(0, head);
            self.body.pop();
            false
        }
    }
}
