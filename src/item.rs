use crate::game::Game;

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    description: String,
    is_collectible: bool,
    is_usable: bool,
    effect: Option<fn(&mut Game)>,
}