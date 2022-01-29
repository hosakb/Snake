use std::io::Stdout;
use std::thread::sleep;
use std::time::Duration;

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, SetSize,
};
use crossterm::ExecutableCommand;
use rand::Rng;

use crate::point::Point;
use crate::snake::Snake;

const MAX_INTERVAL: u16 = 400;
const MIN_INTERVAL: u16 = 700;
const MAX_SPEED: u16 = 20;

// 60
// 30

pub struct Game {
    stdout: Stdout,
    original_terminal_size: (u16, u16),
    width: u16,
    height: u16,
    food: Option<Point>,
    snake: Snake,
    speed: u16,
    pub score: u16,
    boarder: Vec<Point>,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let original_terminal_size: (u16, u16) = size().unwrap();
        Game {
            stdout,
            original_terminal_size,
            width,
            height,
            food: None,
            snake: Snake::new(width, height),
            speed: 0,
            score: 0,
            boarder: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let (cols, rows) = size().unwrap();
        self.setup_ui();
        self.render();

        loop {
            // let duration = Duration::from_millis(500);
            // sleep(duration);
            println!("{}", self.snake.body.first().unwrap());
            self.snake.get_direc();
            self.snake.change_direc();

            if self.snake.eat(self.food.unwrap()) {
                self.new_food();
            }

            if self.crossed() || self.hit_wall() {
                break;
            }
        }

        self.clean_up(cols, rows);
    }

    fn new_food(&mut self) {
        loop {
            let x = rand::thread_rng().gen_range(1..self.width - 1);
            let y = rand::thread_rng().gen_range(1..self.height - 1);
            let food = Point::new(x, y);

            if !self.snake.body.contains(&food) {
                self.food = Some(food);
                break;
            }
        }

        self.draw_food()
    }

    fn setup_ui(&mut self) {
        // enable_raw_mode().unwrap();
        self.stdout
            // .execute(EnterAlternateScreen)
            // .unwrap()
            .execute(SetSize(self.width, self.height))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(Hide)
            .unwrap();
    }

    fn render(&mut self) {
        self.draw_borders();
        self.new_food();
        self.draw_snake();
    }

    fn draw_borders(&mut self) {
        for x in 0..self.width {
            self.stdout
                .execute(MoveTo(x, 0))
                .unwrap()
                .execute(SetForegroundColor(Color::DarkGrey))
                .unwrap()
                .execute(Print("█".to_string()))
                .unwrap();
            self.boarder.push(Point::new(x, 0));
            self.stdout
                .execute(MoveTo(x, self.height - 1))
                .unwrap()
                .execute(SetForegroundColor(Color::DarkGrey))
                .unwrap()
                .execute(Print("█".to_string()))
                .unwrap();
            self.boarder.push(Point::new(x, self.height - 1));
        }

        for y in 0..self.height {
            self.stdout
                .execute(MoveTo(0, y))
                .unwrap()
                .execute(SetForegroundColor(Color::DarkGrey))
                .unwrap()
                .execute(Print("█".to_string()))
                .unwrap();
            self.stdout
                .execute(MoveTo(1, y))
                .unwrap()
                .execute(SetForegroundColor(Color::DarkGrey))
                .unwrap()
                .execute(Print("█".to_string()))
                .unwrap();
            self.boarder.push(Point::new(1, y));
            self.stdout
                .execute(MoveTo(self.width - 2, y))
                .unwrap()
                .execute(SetForegroundColor(Color::DarkGrey))
                .unwrap()
                .execute(Print("█".to_string()))
                .unwrap();
            self.stdout
                .execute(MoveTo(self.width - 1, y))
                .unwrap()
                .execute(SetForegroundColor(Color::DarkGrey))
                .unwrap()
                .execute(Print("█".to_string()))
                .unwrap();
            self.boarder.push(Point::new(self.width - 2, y));
        }
    }

    fn draw_food(&mut self) {
        self.stdout
            .execute(MoveTo::from(self.food.unwrap()))
            .unwrap()
            .execute(Print("🍒".to_string()))
            .unwrap();
    }

    fn draw_snake(&mut self) {
        self.remove_snake();
        let first = *self.snake.body.first().unwrap();
        self.stdout
            .execute(MoveTo::from(first))
            .unwrap()
            .execute(Print("🧿".to_string()))
            .unwrap();

        self.snake.body.clone().into_iter().for_each(|p| {
            if p != first {
                self.stdout
                    .execute(MoveTo::from(p))
                    .unwrap()
                    .execute(SetForegroundColor(Color::Green))
                    .unwrap()
                    .execute(Print("ѻ".to_string()))
                    .unwrap();
            }
        })
    }

    fn remove_snake(&mut self) {
        self.snake.body.clone().into_iter().for_each(|p| {
            self.stdout
                .execute(MoveTo::from(p))
                .unwrap()
                .execute(SetForegroundColor(Color::Green))
                .unwrap()
                .execute(Print("".to_string()))
                .unwrap();
        })
    }

    fn clean_up(&mut self, cols: u16, rows: u16) {
        self.stdout
            .execute(SetSize(cols, rows))
            .unwrap()
            .execute(Show)
            .unwrap()
            .execute(EnterAlternateScreen)
            .unwrap();
        //disable_raw_mode().unwrap();
    }

    pub fn crossed(&mut self) -> bool {
        let body = self.snake.body.split_off(1);

        if body.contains(self.snake.get_head()) {
            return true;
        }

        false
    }

    pub fn hit_wall(&mut self) -> bool {
        if self.boarder.contains(self.snake.get_head()) {
            return true;
        }

        false
    }
}
