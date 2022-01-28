mod game;
mod point;
mod snake;

use std::io::stdout;

use crate::game::Game;

fn main() {
    println!("Snake");
    let game = Game::new(stdout(), 10, 10).run();
}
