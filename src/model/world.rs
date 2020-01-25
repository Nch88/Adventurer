use super::entities::Object;
use std::collections::HashMap;

use crate::model::actions::{Action, ActionType};

use crate::game::world_builder as wb;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct LocationKey(pub u32);

pub struct World {
    pub locations: HashMap<LocationKey, Location>,
    current_loc_key: LocationKey,
}

impl World {
    pub fn new() -> Self {
        let (locations, current_loc_key) = wb::locations::create_locations();
        World {
            locations,
            current_loc_key,
        }
    }

    pub fn current_loc(&self) -> &Location {
        self.locations
            .get(&self.current_loc_key)
            .unwrap_or_else(|| panic!("Bad location key: {:?}", self.current_loc_key))
    }
}

pub struct Location {
    pub name: String,
    pub description: Description,
    pub objects: Vec<Object>,
}

impl Location {
    pub fn new(name: String, description: Description) -> Self {
        Location {
            name,
            description,
            objects: Vec::new(),
        }
    }

    // TODO: Do not hardcode and return a describe action
    pub fn describe(&self) -> Action {
        Action {
            description: "Bla".to_string(),
            typ: ActionType::Describe(self.description.clone()),
        }
    }
}

#[derive(Default, Clone)]
pub struct Description {
    pub general: String,
    pub sound: Option<String>,
    pub sight: Option<String>,
    pub smell: Option<String>,
}
