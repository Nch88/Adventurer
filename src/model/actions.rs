use crate::model::entities::Character;
use crate::model::world::{Description, Location};
use crate::view;

pub enum ActionType {
    Describe(Description),
}

pub struct Action {
    pub description: String,
    pub typ: ActionType,
    // validate: dyn Fn() -> bool,
}

impl Action {
    pub fn exec(&self) {
        match &self.typ {
            ActionType::Describe(desc) => view::show_description(desc),
        }
    }

    // TODO: Allow dynamic validation methods for actions based on location and character state
    pub fn valid(&self) -> bool {
        // (self.validate)()
        true
    }
}

pub trait ActionV2 {
    // TODO: Return refs
    fn new() -> Self;
    fn short_desc(&self) -> String;
    fn long_desc(&self) -> String;
}

pub trait CharAction {
    fn exec(&self, cha: &mut Character);
    fn valid(&self, cha: &Character) -> bool;
}

pub trait LocAction {
    fn valid(&self, cha: &Location) -> bool;
    fn exec(&self, cha: &mut Location);
}

pub mod character_actions {
    use super::{ActionV2, CharAction};
    use crate::model::entities::{Character, Eyes};

    pub mod physical_attr_actions {
        use super::*;

        pub struct OpenEyes {
            short_desc: String,
            long_desc: String,
        }

        impl ActionV2 for OpenEyes {
            fn new() -> Self {
                OpenEyes {
                    short_desc: "Open your eyes.".to_string(),
                    long_desc: "You open your eyes.".to_string(),
                }
            }

            fn short_desc(&self) -> String {
                self.short_desc.clone()
            }

            fn long_desc(&self) -> String {
                self.long_desc.clone()
            }
        }

        impl CharAction for OpenEyes {
            fn valid(&self, cha: &Character) -> bool {
                match cha.physical_attrs.eyes {
                    Eyes::Closed => true,
                    _ => false,
                }
            }

            fn exec(&self, cha: &mut Character) {
                if !self.valid(&cha) {
                    panic!("Action not valid!");
                }
                cha.physical_attrs.eyes = Eyes::Open;
            }
        }
    }

}

mod action_tests {
    use crate::model::actions::character_actions::physical_attr_actions::OpenEyes;
    use crate::model::actions::{ActionV2, CharAction};
    use crate::model::entities::{Character, Eyes};

    #[test]
    fn open_eyes_a_closed_eyes_open() {
        let mut cha = Character::new("Test".to_string());

        match cha.physical_attrs.eyes {
            Eyes::Closed => (),
            _ => panic!("Eyes not closed!"),
        }

        let open_eyes = OpenEyes::new();

        open_eyes.exec(&mut cha);

        match cha.physical_attrs.eyes {
            Eyes::Open => (),
            _ => panic!("Eyes not open!"),
        }
    }

    #[test]
    fn open_eyes_a_disabled_eyes_are_invalid() {
        let mut cha = Character::new("Test".to_string());
        cha.physical_attrs.eyes = Eyes::Disabled;

        match cha.physical_attrs.eyes {
            Eyes::Disabled => (),
            _ => panic!("Eyes not closed!"),
        }

        let open_eyes = OpenEyes::new();

        assert!(!open_eyes.valid(&cha));
    }
}
