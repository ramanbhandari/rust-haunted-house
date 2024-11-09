use std::collections::HashMap;
use crate::item::Item;
use std::fs;
use serde::Deserialize;
use crate::room;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Room {
    name: String,
    description: String,
    connections: HashMap<String, String>,
    items: HashMap<String, String>,
}

pub fn load_rooms() -> HashMap<String, Room> {
    let data = fs::read_to_string("assets/rooms.json").expect("Unable to load rooms");
    let parsed = serde_json::from_str(&data).expect("Parsing error");
    println!("{:#?}", parsed);
    parsed
}
