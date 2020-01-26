use super::world::LocationKey;

pub enum Eyes {
    Open,
    Closed,
    Disabled,
}

impl Default for Eyes {
    fn default() -> Self {
        Eyes::Closed
    }
}

pub struct PhysicalAttributes {
    pub eyes: Eyes,
}

impl Default for PhysicalAttributes {
    fn default() -> Self {
        PhysicalAttributes {
            eyes: Eyes::default(),
        }
    }
}

pub struct Character {
    pub name: String,
    pub loc_key: LocationKey,
    pub physical_attrs: PhysicalAttributes,
}

impl Character {
    pub fn new(name: String) -> Self {
        Character {
            name,
            loc_key: LocationKey(0),
            physical_attrs: PhysicalAttributes::default(),
        }
    }
}

pub struct Object {
    name: String,
}
