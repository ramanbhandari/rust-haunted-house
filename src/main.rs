mod room;
mod item;
mod player;
mod game;
mod utils;

use crate::player::Player;
use crate::game::Game;

fn main() {
    let player = Player {
        curr_room: "Entrance".to_string(),
        inventory: Vec::new(),
        health: 100,
        sanity: 100
    };

    let mut game = Game::new(player);
    game.run();
}
