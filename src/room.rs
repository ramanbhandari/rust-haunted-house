use std::collections::HashMap;
use crate::item::Item;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Room {
    name: String,
    description: String,
    connections: HashMap<String, String>,
    items: Vec<Item>,
}

pub fn load_rooms() -> HashMap<String, Room> {
    let data = fs::read_to_string("assets/rooms.json").expect("Unable to load rooms");
    serde_json::from_str(&data).expect("Parsing error")
}
