//! This is a very stupid bot, who does not even
//! knows the go rules, he is just an example about how
//! to implement the GoBot trait.

#![allow(unused_variable)]

extern crate gtprust;

use gtprust::api;

struct DummyBot;

impl api::GoBot for DummyBot {

    fn gtp_name(&self) -> String {
        String::from_str("DummyBot")
    }

    fn gtp_version(&self) -> String {
        String::from_str("v0.42")
    }

    fn gtp_clear_board(&mut self) {
        // this bot has no memory
    }

    fn gtp_komi(&mut self, komi: f32) {
        // what is a komi ??
    }

    fn gtp_boardsize(&mut self, size: uint) -> Result<(), api::GTPError> {
        // Board size ? Is it not always 19x19 ?
        Ok(())
    }

    fn gtp_play(&mut self, move: api::ColouredMove) -> Result<(), api::GTPError> {
        // Do whatever you want, I don't care.
        Ok(())
    }

    fn gtp_genmove(&mut self, player: api::Colour) -> api::Move {
        api::Stone(api::Vertex::from_coords(10,10).unwrap()) // Tengen !!!
    }

    fn gtp_showboard(&self) -> Result<(uint, Vec<api::Vertex>, Vec<api::Vertex>, uint, uint), api::GTPError> {
        // a simple random board
        Ok((19,
         vec!(api::Vertex::from_str("B12").unwrap(),api::Vertex::from_str("J2").unwrap(),
              api::Vertex::from_str("H8").unwrap(),api::Vertex::from_str("R18").unwrap()),
         vec!(api::Vertex::from_str("R3").unwrap(),api::Vertex::from_str("F9").unwrap(),
              api::Vertex::from_str("C17").unwrap()),
         3,
         4))
    }
}

fn main() {
    let mut mybot = DummyBot;
    gtprust::main_loop(&mut mybot);
}
