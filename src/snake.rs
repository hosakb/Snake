use std::{time::Duration, clone};

use crossterm::event::{poll, read, Event, KeyCode};
use rand::{seq::IteratorRandom, prelude::SliceRandom};

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
            body: vec![Point{x: width/2, y: height/2}],
            direction: Direction::Right,
        }
    }

    pub fn get_head(&mut self) -> &Point{
        self.body.first().unwrap()
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

    pub fn change_direc(&self) {
        match self.direction {
            Direction::Down => self.get_head().y -= 1,
            Direction::Up => self.get_head().y += 1,
            Direction::Left => self.get_head().x -= 1,
            Direction::Right => self.get_head().x += 1,
        }
    }

    pub fn crossed(&self) -> bool {
        let body = self.body.clone().split_off(1);

        if body.contains(self.get_head()) {
            return true;
        }

        false
    }

    pub fn hit_wall(&self, wall: &[Point]) -> bool{

        if wall.contains(self.get_head()) {
            return true
        }

        false
    }

    pub fn eat(&mut self, food: Point) -> bool {

        if food == self.get_head().clone() {
            self.body.insert(0, food);
            true
        } else {
            self.body.insert(0,self.get_head().clone());
            self.body.pop();
            false
        }
    }
}