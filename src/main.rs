mod room;
mod item;
mod player;
mod game;

use crate::player::Player;
use crate::game::Game;

fn main() {
    let player = Player {
        curr_room: "Entrance".to_string(),
        inventory: Vec::new(),
        health: 100,
        sanity: 100
    };

    let game = Game::new(player);
    game.display_curr_room();
}
