use super::world::LocationKey;

pub struct Character {
    pub name: String,
    pub dungeon_key: LocationKey,
    pub room_key: LocationKey,
}

impl Character {
    pub fn new(name: String) -> Self {
        Character {
            name,
            dungeon_key: LocationKey(0),
            room_key: LocationKey(0)
        }
    }
}

pub struct Object {
    name: String
}