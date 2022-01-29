mod game;
mod point;
mod snake;

use std::io::stdout;

use crate::game::Game;

fn main() {
    Game::new(stdout(), 50, 25).run();
}
