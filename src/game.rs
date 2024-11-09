use std::collections::HashMap;
use serde::Deserialize;
use crate::room::{Room, load_rooms};
use crate::player::Player;
use crate::utils::get_user_input;


#[derive(Debug, Clone, serde::Deserialize)]
pub struct Game {
    rooms: HashMap<String, Room>,
    player: Player,
    active: bool,
}

impl Game {
    pub fn new(player: Player) -> Self {
        let rooms = load_rooms();
        Self {
            rooms,
            player,
            active: true,
        }
    }

    pub fn display_curr_room(&self) {
        let curr_room = self.rooms.get(&self.player.curr_room).unwrap();
        println!("\nYou are in: {}", curr_room.name);
        println!("{}", curr_room.description);

        if !curr_room.items.is_empty() {
            println!("You see: ");
            for item in &curr_room.items {
                println!("- {}: {}", item.name, item.description);
            }
        }
        println!("Available directions: {:?}", curr_room.connections.keys());
    }

    pub fn run(&mut self) {
        while self.active {
            self.display_curr_room();

            let input = get_user_input(">");
            if !self.handle_command(input) {
                break;
            }
        }
    }
    fn handle_command(&mut self, command: String) -> bool {
        let command = command.trim().to_lowercase();
        let command_args: Vec<&str> = command.split_whitespace().collect();

        match command_args[0] {
            "go" => {
                if command_args.len() < 2 {
                    println!("Please specify a command, don't be a ghost (e.g. go north)");
                } else {
                    let direction = command_args[1];
                    self.move_player(direction.trim());
                }
            }
            "pick" => {
                if command_args.len() < 3 {
                    println!("Please specify a command, don't be a ghost (e.g. pick up rusty key)");
                } else {
                    let item_name = command_args[2..].join(" ");
                    self.pick_up_item(&item_name);
                }
            }
            "inventory" => {
                self.show_inventory();
            }
            "exit" => {
                println!("Exiting the haunted house..");
                return false;
            }
            _ => {
                println!("Command is not know. Try maybe go....");
            }
        }
        true
    }

    fn move_player(&mut self, direction: &str) {
        let curr_room = self.rooms.get(&self.player.curr_room).unwrap();
        if let Some(next_room) = curr_room.connections.get(direction) {
            self.player.curr_room = next_room.to_string();
        }
        else {
            println!("That direction is ghosted, you can't go");
        }
    }

    fn pick_up_item(&mut self, item_name: &str) {
        let curr_room = self.rooms.get_mut(&self.player.curr_room).unwrap();
        if let Some(position) = curr_room.items.iter().position(|item| item.name == item_name) {
            let item = curr_room.items.remove(position);
            self.player.inventory.push(item);
            println!("You picked up the {}", item_name);
        } else {
            println!("No such item ghostie");
        }
    }

    fn show_inventory(&self) {
        if self.player.inventory.is_empty() {
            println!("You have no inventory ghostie");
        } else {
            println!("You have:");
            for item in &self.player.inventory {
                println!("- {}:{}", item.name, item.description);
            }
        }
    }
}