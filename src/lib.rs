#![crate_type = "lib"]

use std::io::Write;

pub mod api;
mod bothandler;
mod parsing;
mod boarddrawer;

/// This function is the mail loop of your bot.
/// You must provide it a struct implementing the
/// trait `api::GoBot`, thus providing all the required callbacks.
#[allow(dead_code)]
pub fn main_loop<T: api::GoBot>(bot: &mut T) {
    let handler = bothandler::BotHandler::from_bot(bot);
    let input = std::io::stdin();
    let mut output = std::io::stdout();
    loop {
        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) => { line = "quit".to_string() },
            Ok(_) => (),
            Err(_) => panic!("IO error.")
        };
        let (continue_loop, result) = handler.handle_command(bot, &line);
        match output.write((result + "\n\n").as_bytes()) {
            Err(_) => panic!("IO error."),
            _ => {}
        }
        if !continue_loop {
            break;
        }
    }
}
