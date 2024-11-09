use serde::Deserialize;
use crate::item::Item;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Player {
    curr_room: String,
    inventory: Vec<Item>,
    health: u32,
    sanity: u32,
}