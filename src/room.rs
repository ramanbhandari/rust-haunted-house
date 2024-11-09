use std::collections::HashMap;

pub struct Room {
    name: String,
    description: String,
    connections: HashMap<String, String>,
    items: Vec<Item>,
    is_locked: bool,
    puzzle: Option<Puzzle>
}