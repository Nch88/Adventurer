use crate::model::entities::Character;
use crate::model::world::{Dungeon, Location, LocationKey, Room, World};
use crate::view;

use std::collections::HashMap;

pub struct Game {
    pub world: World,
    pub character: Character,
}

impl Game {
    pub fn new() -> Self {
        let locations = create_locations();
        Game {
            world: World::new(locations),
            character: Character::new("Noname".to_owned()),
        }
    }

    pub fn start(&self) {
        println!("Welcome... To the game!");
        character_creation();
        self.game_loop();
    }

    fn game_loop(&self) {
        let mut action_idx = 0;
        loop {
            self.show_description();
            action_idx = self.prompt_user_action();
            self.update(action_idx);
        }
    }

    fn show_description(&self) {
        let location_key = &self.character.dungeon_key;
        let description = match self
            .world
            .locations
            .get(location_key)
            .expect("Invalid character dungeon key")
        {
            Location::Dungeon(dungeon) => {
                let room_key = &self.character.room_key;
                let room = &dungeon.rooms[room_key.0 as usize];
                room.get_description()
            }
            _ => unimplemented!(),
        };
        view::show_message(&description);
    }

    fn prompt_user_action(&self) -> u8 {
        view::show_message(&"Which action would you like to take?");
        let action_idx = view::prompt_user_input_index();
        // TODO: validate action idx based on room actions
        action_idx
    }

    fn update(&self, action_idx: u8) {
        // TODO: Update state
    }
}

fn create_locations() -> HashMap<LocationKey, Location> {
    let mut locations = HashMap::new();

    let mut dungeon0 = Dungeon::new();

    let mut room0 = Room::new("Starting room".to_owned());

    room0.state_descriptions.push("You wake up as if from a deep sleep... You feel yourself lying down on an uncomfortable bed. You eyes are still closed.".to_owned());
    // TODO: Allow for multi-line descriptions

    dungeon0.rooms.push(room0);

    let loc0 = Location::Dungeon(dungeon0);

    locations.insert(LocationKey(0), loc0);
    locations
}

fn character_creation() -> Character {
    let welcome = "First you need to create a character.";
    view::show_message(&welcome);

    view::show_message(&"What name do you chose:");
    let name = view::prompt_user_input_string();

    Character::new(name)
}
