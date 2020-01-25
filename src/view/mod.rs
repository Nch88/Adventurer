use std::fmt::Display;
use std::io;

pub fn show_message<M>(msg: &M)
where
    M: Display,
{
    print_msg(msg);
}

pub fn show_prompt<M>(msg: &M)
where
    M: Display,
{
    print_prompt(msg);
}

pub fn prompt_user_input_string() -> String {
    let mut inp = String::new();

    loop {
        match io::stdin().read_line(&mut inp) {
            Ok(_) => break,
            Err(_) => print_meta(&"I did not understand that, please try again:"),
        }
    }

    inp.trim().to_string()
}

pub fn prompt_user_input_index() -> u8 {
    let mut guess = String::new();

    loop {
        match io::stdin().read_line(&mut guess) {
            Ok(_) => match guess.parse::<u8>() {
                Ok(v) => break v,
                Err(_) => print_meta(&"That does not seem to be a valid index, please try again:"),
            },
            Err(_) => print_meta(&"I did not understand that, please try again:"),
        }
    }
}

fn print_prompt<M>(msg: &M)
where
    M: Display,
{
    print_w_prefix(msg, "?>");
}

fn print_meta<M>(msg: &M)
where
    M: Display,
{
    print_w_prefix(msg, "!>");
}

fn print_msg<M>(msg: &M)
where
    M: Display,
{
    print_w_prefix(msg, " >");
}

fn print_w_prefix<M>(msg: &M, prefix: &str)
where
    M: Display,
{
    println!("{} {}", prefix, msg);
}
