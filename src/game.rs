use std::collections::HashMap;
use crate::room::Room;
use crate::player::Player;

pub struct Game {
    rooms: HashMap<String, Room>,
    player: Player,
    active: bool,
}