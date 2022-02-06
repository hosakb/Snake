use std::ops::IndexMut;
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use rand::Rng;

use crate::point::Point;
use crate::snake::{Direction, Snake};
use crate::terminal::Terminal;

const MAX_INTERVAL: u16 = 400;
const MIN_INTERVAL: u16 = 700;
const MAX_SPEED: u16 = 20;

// 60
// 30

pub struct Game {
    terminal: Terminal,
    food: Option<Point>,
    snake: Snake,
    speed: u16,
    pub score: u16,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        Game {
            terminal: Terminal::new(width, height, Vec::new()),
            food: None,
            snake: Snake::new(width, height),
            speed: 0,
            score: 0,
        }
    }

    pub fn run(&mut self) {

        self.terminal.setup_ui();

        loop {
            if self.get_user_input() {
                break;
            }
            self.update_values();
            self.terminal.render(self.snake.clone(), self.food.unwrap());
            if self.check() {
                break;
            }
        }

        self.terminal.clean_up_ui();
    }

    fn new_food(&mut self) {
        loop {
            let x = rand::thread_rng().gen_range(1..self.terminal.width - 2); // width
            let y = rand::thread_rng().gen_range(1..self.terminal.height - 3); // height
            let food = Point::new(x, y);

            if !self.snake.body.contains(&food) {
                self.food = Some(food);
                break;
            }
        }
    }

    fn check(&mut self) -> bool {
        if self.crossed() || self.hit_wall() {
            return true;
        }

        false
    }

    fn crossed(&self) -> bool {
        let body = self.snake.body.clone().split_off(1);

        if body.contains(self.snake.get_head()) {
            return true;
        }

        false
    }

    fn hit_wall(&self) -> bool {
        if self.terminal.boarder.contains(self.snake.get_head()) {
            return true;
        }

        false
    }

    fn get_user_input(&mut self) -> bool{
        if poll(Duration::from_millis(500)).expect("Error polling key press") {
            if let Event::Key(event) = read().expect("Error reading keys") {
                self.snake.direction = match event.code {
                    KeyCode::Down => Direction::Down,
                    KeyCode::Up => Direction::Up,
                    KeyCode::Left => Direction::Left,
                    KeyCode::Right => Direction::Right,
                    KeyCode::Char('q') => return true,
                    _ => self.snake.direction,
                }
            }
        }

        false
    }

    fn update_values(&mut self) {
        if self.food.is_none() {
            self.new_food();
        }

        self.move_or_eat();
    }

    fn move_or_eat(&mut self) {
        let food = self.food.unwrap();
        let head = *self.snake.get_head();

        if head == food {
            self.snake.body.push(self.food.take().unwrap())
        }

        self.move_snake_part();
    }

    fn move_snake_part(&mut self) {
        for i in self.snake.body.len() - 1..0 {
            if i != self.snake.body.len() - 1 {
                self.snake.body.swap(i, i - 1);
            } else {
                let mut head = *self.snake.body.first().unwrap();

                match self.snake.direction {
                    Direction::Down => head.y += 1,
                    Direction::Up => head.y -= 1,
                    Direction::Left => head.x -= 1,
                    Direction::Right => head.x += 1,
                }

                *self.snake.body.index_mut(i) = head;
            }
        }
    }
}
