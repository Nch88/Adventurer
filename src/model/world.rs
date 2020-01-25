use super::entities::Object;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub struct LocationKey(pub u32);

pub enum Location {
    Dungeon(Dungeon),
}

pub struct World {
    pub locations: HashMap<LocationKey, Location>,
}

impl World {
    pub fn new(locations: HashMap<LocationKey, Location>) -> Self {
        World { locations }
    }
}

pub struct Dungeon {
    pub rooms: Vec<Room>,
}

impl Dungeon {
    pub fn new() -> Self {
        Dungeon { rooms: Vec::new() }
    }
}

pub struct Room {
    pub name: String,
    pub state: u8,
    pub state_descriptions: Vec<String>,
    pub objects: Vec<Object>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Room{
            name,
            state: 0,
            state_descriptions: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn get_description(&self) -> &str {
        &self.state_descriptions[self.state as usize]
    }
}