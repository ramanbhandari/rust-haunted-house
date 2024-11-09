use std::collections::HashMap;
use serde::Deserialize;
use crate::room::{Room, load_rooms};
use crate::player::Player;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Game {
    rooms: HashMap<String, Room>,
    player: Player,
    active: bool,
}

impl Game {
    pub fn new(player: Player) -> Self {
        let rooms = load_rooms();
        Self {
            rooms,
            player,
            active: true,
        }
    }

    pub fn display_curr_room(&self) {
        let curr_room = self.rooms.get(&self.player.curr_room).unwrap();
        println!("\nYou are in: {}", curr_room.name);
        println!("{}", curr_room.description);

        if !curr_room.items.is_empty() {
            println!("You see: ");
            for item in &curr_room.items {
                println!("- {}: {}", item.name, item.description);
            }
        }
        println!("Available directions: {:?}", curr_room.connections.keys());
    }
}