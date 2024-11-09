use serde::Deserialize;
use crate::game::Game;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub is_collectible: bool,
    pub is_usable: bool,
}