#![crate_type = "lib"]

mod api;
mod bothandler;
mod parsing;

#[allow(dead_code)]
pub fn main_loop<T: api::GoBot>(bot: &mut T) {
    let handler = bothandler::BotHandler::from_bot(bot);
}
