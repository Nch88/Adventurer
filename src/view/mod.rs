use std::fmt::Display;
use std::io;

pub fn show_message<M>(msg: &M)
where
    M: Display,
{
    println!("{}", msg)
}

pub fn prompt_user_input_string() -> String {
    let mut guess = String::new();

    loop {
        match io::stdin().read_line(&mut guess) {
            Ok(_) => break,
            Err(_) => println!("I did not understand that, please try again:",),
        }
    }

    guess
}

pub fn prompt_user_input_index() -> u8 {
    let mut guess = String::new();

    loop {
        match io::stdin().read_line(&mut guess) {
            Ok(_) => match guess.parse::<u8>() {
                Ok(v) => break v,
                Err(_) => println!("That does not seem to be a valid index, please try again:",),
            },
            Err(_) => println!("I did not understand that, please try again:",),
        }
    }
}
