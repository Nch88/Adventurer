use crate::model::entities::Character;
use crate::view;

pub fn character_creation() -> Character {
    let welcome = "First you need to create a character.";
    view::show_message(&welcome);

    // TODO: Combine print and user input into one function
    view::show_prompt(&"What name do you chose:");
    let name = view::prompt_user_input_string();

    view::show_message(&format!(
        "Ahh, the mighty {}. Tales of your adventures have been told throughout the land.",
        name
    ));
    view::show_message(&"Are you ready for one more?");
    view::show_message(&"");
    Character::new(name)
}
