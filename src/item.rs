use serde::Deserialize;
use crate::game::Game;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Item {
    name: String,
    description: String,
    is_collectible: bool,
    is_usable: bool,
}