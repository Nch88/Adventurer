use crate::view;

pub mod startup {
    use super::*;

    pub fn welcome_msg() {
        view::show_message(&"Welcome adventurer... To the great game!");
        view::show_message(&"Are you ready to play again?");
        view::show_message(&"");
    }
}

pub mod locations {
    use crate::model::world::{Description, Location, LocationKey};

    use std::collections::HashMap;

    pub fn create_locations() -> (HashMap<LocationKey, Location>, LocationKey) {
        let mut locations = HashMap::new();

        let loc0 = Location::new("Starting room".to_owned(), Description{
            general: "You wake up as if from a deep sleep... You feel yourself lying down on an uncomfortable bed. You eyes are still closed.".to_owned(),
            ..Default::default()
        });
        let loc_key0 = LocationKey(0);

        // TODO: Allow for multi-line descriptions

        locations.insert(loc_key0, loc0);
        (locations, loc_key0)
    }
}
