use crate::model::actions::Action;
use crate::model::entities::Character;
use crate::model::world::{Location, World};
use crate::view;

pub mod world_builder;
use world_builder as wb;

mod character_creater;
use character_creater as char_cre;

pub struct Game {
    pub world: World,
    pub character: Character,
    pub action: Action, // TODO: Do not copy around actions
}

impl Game {
    pub fn new() -> Self {
        let world = World::new();
        let action = world.current_loc().describe();
        Game {
            world,
            character: Character::new("Noname".to_owned()),
            action,
        }
    }

    pub fn start(&self) {
        wb::startup::welcome_msg();
        char_cre::character_creation();
        self.game_loop();
    }

    fn game_loop(&self) {
        let mut action_idx = 0;
        loop {
            self.action.exec();
            action_idx = self.prompt_user_action();
            self.update(action_idx);
        }
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
