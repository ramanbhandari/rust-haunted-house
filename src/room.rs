use std::collections::HashMap;
use crate::item::Item;
use std::fs;
use serde::Deserialize;
use crate::room;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub connections: HashMap<String, String>,
    pub items: Vec<Item>,
}

pub fn load_rooms() -> HashMap<String, Room> {
    let data = fs::read_to_string("assets/rooms.json").expect("Unable to load rooms");
    serde_json::from_str(&data).expect("Parsing error")
}
