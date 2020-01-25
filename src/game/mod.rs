use crate::model::entities::Character;
use crate::model::world::{Location, World};
use crate::view;

mod world_builder;
use world_builder as wb;

mod character_creater;
use character_creater as chacre;

pub struct Game {
    pub world: World,
    pub character: Character,
}

impl Game {
    pub fn new() -> Self {
        let locations = wb::locations::create_locations();
        Game {
            world: World::new(locations),
            character: Character::new("Noname".to_owned()),
        }
    }

    pub fn start(&self) {
        wb::startup::welcome_msg();
        chacre::character_creation();
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
                // TODO: Remove notion of Rooms and just have Locations?
                // A location should have (be?) a Trait that can generate a list of actions based on its own state and the state of the player character.
                let room_key = &self.character.room_key;
                let room = &dungeon.rooms[room_key.0 as usize];
                room.get_description()
            }
            _ => unimplemented!(),
        };
        view::show_message(&description);
    }

    fn prompt_user_action(&self) -> u8 {
        // TODO: Move prompt to view
        // Create func that takes a slice of actions, shows the prompt, lists the actions and return a valid action.
        view::show_prompt(&"Which action would you like to take?");
        let action_idx = view::prompt_user_input_index();
        // TODO: validate action idx based on room actions
        action_idx
    }

    fn update(&self, action_idx: u8) {
        // TODO: Update state
    }
}
