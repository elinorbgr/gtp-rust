#![crate_type = "lib"]

pub mod api;
mod bothandler;
mod parsing;

/// This function is the mail loop of your bot.
/// You must provide it a struct implementing the
/// trait `api::GoBot`, thus providing all the required callbacks.
#[allow(dead_code)]
pub fn main_loop<T: api::GoBot>(bot: &mut T) {
    let handler = bothandler::BotHandler::from_bot(bot);
    let mut input = std::io::stdio::stdin();
    let mut output = std::io::stdio::stdout();
    loop {
        let line: String = match input.read_line() {
            Ok(txt) => txt,
            Err(_) => fail!("IO error.")
        };
        // convert line to ascii slice
        let ascii_input: Vec<Ascii> = match line.as_slice().to_ascii_opt() {
            Some(txt) => Vec::from_slice(txt),
            None => vec!('#'.to_ascii())
        };
        let (continue_loop, result) = handler.handle_command(bot, ascii_input.as_slice());
        match output.write(result.append("\n\n").as_bytes()) {
            Err(_) => fail!("IO error."),
            _ => {}
        }
        if !continue_loop {
            break;
        }
    }
}
