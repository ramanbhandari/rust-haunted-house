use serde::Deserialize;
use crate::item::Item;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Player {
    pub curr_room: String,
    pub inventory: Vec<Item>,
    pub health: u32,
    pub sanity: u32,
}