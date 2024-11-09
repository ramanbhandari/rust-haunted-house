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
}