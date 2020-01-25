use crate::model::world::Description;
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
