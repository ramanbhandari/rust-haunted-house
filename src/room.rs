use std::collections::HashMap;
use crate::item::Item;
pub struct Room {
    name: String,
    description: String,
    connections: HashMap<String, String>,
    items: Vec<Item>,
    is_locked: bool,
}