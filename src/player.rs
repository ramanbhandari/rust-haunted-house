use crate::item::Item;

pub struct Player {
    curr_room: String,
    inventory: Vec<Item>,
    health: u32,
    sanity: u32,
}