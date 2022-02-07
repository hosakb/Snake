use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use rand::Rng;

use crate::point::Point;
use crate::snake::{Direction, Snake};
use crate::terminal::Terminal;

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
            terminal: Terminal::new(width, height),
            food: None,
            snake: Snake::new(width, height),
            speed: 0,
            score: 0,
        }
    }

    pub fn run(&mut self) {
        self.terminal.setup_ui();
        self.new_food();

        loop {
            if self.get_user_input() {
                break;
            }
            self.move_or_eat();

            self.terminal.render(self.snake.clone(), self.food.unwrap(), self.score);

            if self.crossed() || self.hit_wall() {
                break;
            }
        }

        self.terminal.clean_up_ui();
    }

    fn new_food(&mut self) {
        loop {
            let x = rand::thread_rng().gen_range(2..self.terminal.width - 2); // width
            let y = rand::thread_rng().gen_range(1..self.terminal.height - 3); // height
            let food = Point::new(x, y);

            if !self.snake.body.contains(&food) || x % 2 == 0 {
                self.food = Some(food);
                break;
            }
        }
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

    fn get_user_input(&mut self) -> bool {
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

    fn move_or_eat(&mut self) {
        let food = self.food.unwrap();
        let head = *self.snake.get_head();

        self.move_snake();

        if head == food {
            self.terminal.remove_food(food);
            self.snake.body.push(self.food.take().unwrap());
            self.new_food();
            self.score += 1;
        }
    }

    fn move_snake(&mut self) {
        let mut new_head = *self.snake.body.first_mut().unwrap();

        match self.snake.direction {
            Direction::Down => new_head.y += 1,
            Direction::Up => new_head.y -= 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        self.snake.body.insert(0, new_head);
        self.snake.body.pop();
    }
}
