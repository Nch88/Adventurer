use crate::view;

pub mod startup {
    use super::*;

    pub fn welcome_msg() {
        view::show_message(&"Welcome adventurer... To the great game!");
        view::show_message(&"Are you ready to play again?");
        view::show_message(&"");
    }
}

pub mod actions {
    use crate::model::actions::character_actions::physical_attr_actions::OpenEyes;
    use crate::model::actions::{ActionV2, ActionKey};
    use std::collections::HashMap;

    const open_eyes_key: &str = &"open_eyes_action";

    // pub fn gen_actions() -> HashMap<ActionKey, Box<dyn ActionV2>> {
    //     HashMap::new()
    // }
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
