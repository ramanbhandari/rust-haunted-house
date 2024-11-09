use std::collections::HashMap;
use crate::room::{Room, load_rooms};
use crate::player::Player;

pub struct Game {
    rooms: HashMap<String, Room>,
    player: Player,
    active: bool,
}

impl Game {
    pub fn new() -> Self {
        let rooms = load_rooms();

    }
}