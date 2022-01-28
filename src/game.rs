use std::io::Stdout;
use std::os::windows::process::CommandExt;

use crossterm::cursor::{Hide, Show, MoveTo};
use crossterm::style::{SetForegroundColor, Color, Print, SetBackgroundColor};
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
    score: u16,
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
        }
    }

    pub fn run(&self) {
        let (cols, rows) = size().unwrap();
        self.new_food();
        self.setup_ui();
        self.render();

        loop {
            self.snake.get_direc();
            self.snake.change_direc();

            if self.snake.eat(self.food.unwrap()) {
                self.new_food();
            }

            if self.snake.crossed() || self.snake.hit_wall() {
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
    }

    fn setup_ui(&self) {
        enable_raw_mode().unwrap();
        self.stdout
            .execute(EnterAlternateScreen)
            .unwrap()
            .execute(SetSize(self.width + 4, self.height + 4))
            .unwrap()
            .execute(Clear(ClearType::All))
            .unwrap()
            .execute(Hide)
            .unwrap();
    }

    fn render(&self) {
        self.draw_borders();
        self.draw_food();
        self.draw_snake();
    }

    fn draw_borders(&self) {
        for x in 2..self.width + 2{
            self.stdout.execute(MoveTo(x, 2)).unwrap().execute(SetForegroundColor(Color::DarkGrey)).unwrap().execute(Print("█".to_string())).unwrap();
            self.stdout.execute(MoveTo(x, self.height - 3)).unwrap().execute(SetForegroundColor(Color::DarkGrey)).unwrap().execute(Print("█".to_string())).unwrap();
        }

        for y in 2..self.height + 2{
            self.stdout.execute(MoveTo(2, y)).unwrap().execute(SetForegroundColor(Color::DarkGrey)).unwrap().execute(Print("█".to_string())).unwrap();
            self.stdout.execute(MoveTo(3, y)).unwrap().execute(SetForegroundColor(Color::DarkGrey)).unwrap().execute(Print("█".to_string())).unwrap();
            self.stdout.execute(MoveTo(self.width - 3, y)).unwrap().execute(SetForegroundColor(Color::DarkGrey)).unwrap().execute(Print("█".to_string())).unwrap();
            self.stdout.execute(MoveTo(self.width - 2, y)).unwrap().execute(SetForegroundColor(Color::DarkGrey)).unwrap().execute(Print("█".to_string())).unwrap();
        }
    }

    fn draw_food(&self) {
        self.stdout.execute(MoveTo::from(self.food.unwrap())).unwrap();
    }

    fn draw_snake(&self) {
        self.snake.body.iter().for_each(|p|{
            let first = self.snake.body.first().unwrap().clone();
            if p == first{
                self.stdout.execute(MoveTo::from(first)).unwrap().execute(SetForegroundColor(Color::
                
                
                
                )).unwrap().execute(Print("█".to_string())).unwrap();
            }
        })
    }

    fn clean_up(&self, cols: u16, rows: u16) {
        self.stdout
            .execute(SetSize(cols, rows))
            .unwrap()
            .execute(Show)
            .unwrap()
            .execute(EnterAlternateScreen)
            .unwrap();
        disable_raw_mode().unwrap();
    }
}
