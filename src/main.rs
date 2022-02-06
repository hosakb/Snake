mod game;
mod point;
mod snake;
mod terminal;

use crate::game::Game;

fn main() {
    Game::new(50, 25).run();
}
