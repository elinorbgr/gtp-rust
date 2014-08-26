//! This API contains all you will need to interface your
//! your bot algorithm with the GTPv2 protocol.
//! Your main task will be to implement the GoBot trait.

use std::vec::Vec;

/// Contains all the possible errors your bot
/// may return to the library.
/// Be careful, any callback returning an error it is not
/// supposed to will cause the lib to `fail!()`.
pub enum GTPError {
    NotImplemented,
    InvalidBoardSize,
    InvalidMove,
    BadVertexList,
    BoardNotEmpty,
    CannotUndo,
    CannotScore,
}

/// Represents a player, Black or White.
#[deriving(PartialEq,Show)]
pub enum Colour {
    Black,
    White
}

/// Represents a vertex of the board.
/// Note that board size is at most 25x25.
#[deriving(PartialEq,Show)]
pub struct Vertex {
    x: u8, // letter
    y: u8  // number
}

/// Represents a move, either placing a stone, passing or resigning.
#[deriving(PartialEq,Show)]
pub enum Move {
    Stone(Vertex),
    Pass,
    Resign
}

/// Represents a move associated with a player.
#[deriving(PartialEq,Show)]
pub struct ColouredMove {
    pub player: Colour,
    pub move: Move
}

/// The status of a stone : alive, dead or seki.
#[deriving(PartialEq,Show)]
pub enum StoneStatus {
    Alive,
    Seki,
    Dead
}

/// This is the trait ised by the library to callback your bot.
/// You must implement some functions, the provided one correspond
/// to the optionnal commands of the protocol. If you want to
/// implement them, simply override them. If you do not, the library
/// will not report them as available.
pub trait GoBot {

    /// The name of your bot (ex : "My super Bot")
    fn gtp_name(&self) -> String;

    /// The version of your bot (ex : "v2.3-r5")
    fn gtp_version(&self) -> String;

    // Any function returning a GTPError that it is not supposed
    // to return will be fatal to the framework.

    // Basic functions, must be implemented

    /// Clears the board, can never fail.
    fn gtp_clear_board(&mut self) -> ();

    /// Sets the komi, can never fail, must accept absurd values.
    fn gtp_komi(&mut self, komi: f32) -> ();

    /// Sets the board size.
    /// Returns `Err(InvalidBoardSize)` if the size is not supported.
    /// The protocol cannot handle board sizes > 25x25.
    fn gtp_boardsize(&mut self, size: uint) -> Result<(), GTPError>;

    /// Plays the provided move on the board.
    /// Returns `Err(InvalidMove)` is the move is invalid.
    /// The protocol does not forbid the same player player twice in a row.
    fn gtp_play(&mut self, move: ColouredMove) -> Result<(), GTPError>;

    /// Ask the bot for a move for the chose player.
    /// Cannot fail, the bot must provide a move even if the last
    /// played move is of the same colour.
    /// Plays the move in the internal representation of the game of the bot.
    fn gtp_genmove(&mut self, player: Colour) -> Move;

    // Optional functions, if not iplemented, the corresponding
    // commands will not be activated
    // All these functions will be called once by the framework
    // at startup, then clear_board will be called

    /// Asks the bot for a move for the chosen player.
    /// Must be deterministic, and must not actually play the move.
    /// Should always return `Ok(Move)`, never raise any error.
    #[allow(unused_variable)]
    fn gtp_genmove_regression(&self, player: Colour) -> Result<Move, GTPError> {
        Err(NotImplemented)
    }

    /// Undo last move if possible.
    /// If not, return `Err(CannotUndo)`.
    /// If undo is never possible, should not be implemented.
    #[allow(unused_variable)]
    fn gtp_undo(&mut self) -> Result<(), GTPError> {
        Err(NotImplemented)
    }
    /// The bot places handicap stones for black
    /// according to pre-defined patterns, see specification of GTPv2.
    /// Returns a vertex of choosen stones.
    /// Can fail with `Err(boardNotEmpty)`.
    /// The library garanties `number` will always be between 2 and 9 included.
    #[allow(unused_variable)]
    fn gtp_fixed_handicap(&mut self, number: uint) -> Result<Vec<Vertex>, GTPError> {
        Err(NotImplemented)
    }

    /// The bot places its handicap stones
    /// and returns a vector of Vertexes.
    /// It can place less stones if the asked number is too high.
    /// Can fail with `Err(BoardNotEmpty)` if board isn't empty
    #[allow(unused_variable)]
    fn gtp_place_free_handicap(&mut self, number: uint) -> Result<Vec<Vertex>, GTPError> {
        Err(NotImplemented)
    }

    /// Uses the provided list as handicap stones for black.
    /// Fails with `Err(BoardNotEmpty)` if board isn't empty.
    /// Fails with `Err(BadVertexList)` if the vertex list is unusable
    /// (two stones at the same place, or stones outside the board).
    #[allow(unused_variable)]
    fn gtp_set_free_handicap(&mut self, stones: &[Vertex]) -> Result<(), GTPError> {
        Err(NotImplemented)
    }

    /// Sets the time settings for the game.
    /// It is only informative, the bot should count it's own time,
    /// but the controller is supposed to enforce it.
    /// Time are give in minute, should never fail.
    #[allow(unused_variable)]
    fn gtp_time_settings(&mut self, main_time: uint, byoyomi_time: uint, byoyomi_stones: uint) -> Result<(), GTPError> {
        Err(NotImplemented)
    }

    /// Returns a vector of stones of both color in the given status,
    /// in the opinion of the bot.
    /// Should never fail.
    #[allow(unused_variable)]
    fn gtp_final_status_list(&self, status: StoneStatus) -> Result<Vec<Vertex>, GTPError> {
        Err(NotImplemented)
    }

    /// Computes the bot's calculation of the final score.
    /// If it is a draw, float value must be 0 and colour is not important.
    /// Can fail with èErr(CannotScore)`.
    #[allow(unused_variable)]
    fn gtp_final_score(&self) -> Result<(f32, Colour), GTPError> {
        Err(NotImplemented)
    }

    /// Returns a description of the board as saw by the bot :
    /// (boardsize, black_stones, white_stones, black_captured_count, white_captured_count).
    /// Should never fail.
    #[allow(unused_variable)]
    fn gtp_showboard(&self) -> Result<(uint, Vec<Vertex>, Vec<Vertex>, uint, uint), GTPError> {
        Err(NotImplemented)
    }

    /// Allow you to handle custom commands. Returns (succes, output).
    #[allow(unused_variable)]
    fn gtp_custom_command(&mut self, command: &str, args: &str) -> (bool, String) {
        (false, String::from_str("invalid command"))
    }

    /// Returns true if the given custom command is known.
    #[allow(unused_variable)]
    fn gtp_known_custom_command(&self, command: &str) -> bool {
        false
    }

    /// Returns the list of you custom commands.
    #[allow(unused_variable)]
    fn gtp_list_custom_commands(&self) -> Vec<String> {
        Vec::new()
    }
}

// Vertex implementation for messing with strings
impl Vertex {
    /// Creates a vertex from 2 numerical coords.
    /// Both must be between 1 and 25.
    pub fn from_coords(x: u8, y:u8) -> Option<Vertex> {
        if x == 0 || x > 25 || y == 0 || y > 25 {
            None
        } else {
            Some(Vertex{x: x, y: y})
        }
    }

    /// Creates a vertex from board coordinates (from A1 to Z25).
    /// Remember that letter I is banned.
    pub fn from_str(text: &str) -> Option<Vertex> {
        if text.len() < 2 || text.len() > 3 {
            return None;
        }
        let mut x: u8 = text.as_bytes()[0];
        if x < ('A' as u8) || x > ('Z' as u8) || (x as char) == 'I' {
            return None;
        }
        x -= ('A' as u8) - 1;
        if x > 9 {
            x -= 1;
        } // eliminate 'I'
        let number = from_str::<u8>(text.slice_from(1));
        let mut y: u8 = 0;
        match number {
            Some(num) => y = num,
            _ => (),
        }
        if y == 0 || y > 25 {
            return None;
        }
        Some(Vertex{x: x, y: y})
    }

    /// Returns a tuple of coordinates.
    pub fn to_coords(&self) -> (u8, u8) {
        (self.x, self.y)
    }

    /// Returns the string representation of this vertex (ex: G12).
    pub fn to_string(&self) -> String {
        let mut letter: u8 = 'A' as u8;
        if self.x >= 9 {
            // eliminate 'I'
            letter += self.x;
        } else {
            letter += self.x-1;
        }
        format!("{:c}{:u}", letter as char, self.y)
    }
}

impl Move {
    /// Returns a string representation of the move compatible with
    /// GTPv2.
    pub fn to_string(&self) -> String {
        match *self {
            Stone(vrtx) => vrtx.to_string(),
            Pass => String::from_str("pass"),
            Resign => String::from_str("resign")
        }
    }
}

impl Colour {
    /// Returns a string representation of the color compatible with
    /// GTPv2.
    pub fn to_string(&self) -> String {
        match *self {
            White => String::from_str("white"),
            Black => String::from_str("black")
        }
    }
}

impl ColouredMove {
    /// Returns a string representation of the colored move compatible
    /// with GTPv2.
    pub fn to_string(&self) -> String {
        self.player.to_string().append(self.move.to_string().as_slice())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn vertex_to_string() {
        let vrtx1 = super::Vertex::from_coords(8u8, 7u8).unwrap();
        assert_eq!(vrtx1.to_string().as_slice(), "H7");
        let vrtx2 = super::Vertex::from_coords(9u8, 13u8).unwrap();
        assert_eq!(vrtx2.to_string().as_slice(), "J13");
        let vrtx3 = super::Vertex::from_coords(19u8, 1u8).unwrap();
        assert_eq!(vrtx3.to_string().as_slice(), "T1");
    }

    #[test]
    fn string_to_vertex() {
        let vrtx1 = super::Vertex::from_str("C7").unwrap();
        assert_eq!(vrtx1.to_coords(), (3u8, 7u8));
        let vrtx2 = super::Vertex::from_str("J11").unwrap();
        assert_eq!(vrtx2.to_coords(), (9u8, 11u8));
        let vrtx3 = super::Vertex::from_str("Z25").unwrap();
        assert_eq!(vrtx3.to_coords(), (25u8, 25u8));
    }

    #[test]
    #[should_fail]
    fn too_big_coordinates() {
        let vrtx = super::Vertex::from_coords(26u8, 13u8).unwrap();
        assert_eq!(vrtx.to_coords(), (26u8, 13u8));
    }

    #[test]
    #[should_fail]
    fn invalid_string() {
        let vrtx = super::Vertex::from_str("I13").unwrap();
        assert_eq!(vrtx.to_coords(), (9u8, 13u8));
    }

}
