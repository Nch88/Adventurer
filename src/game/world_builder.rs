use crate::view;

pub mod startup {
    use super::*;

    pub fn welcome_msg() {
        view::show_message(&"Welcome adventurer... To the game!");
        view::show_message(&"Are you ready to play again?");
    }
}

pub mod locations {
    use crate::model::world::{Dungeon, Location, LocationKey, Room};

    use std::collections::HashMap;

    pub fn create_locations() -> HashMap<LocationKey, Location> {
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
}
