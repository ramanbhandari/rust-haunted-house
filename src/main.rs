mod room;
mod item;
mod player;
mod game;

use crate::player::Player;
use crate::game::Game;

fn main() {
    println!("{:#?}", room::load_rooms());
    println!("Hello, world!");
}
