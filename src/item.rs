use crate::game::Game;

pub struct Item {
    name: String,
    description: String,
    is_collectible: bool,
    is_usable: bool,
    effect: Option<fn(&mut Game)>,
}