pub enum GTPError {
    NotImplemented,
    InvalidBoardSize,
    InvalidMove,
    BadVertexList,
    BoardNotEmpty,
    CannotUndo,
}

#[deriving(PartialEq,Show)]
pub enum Colour {
    Black,
    White
}

#[allow(dead_code)]
#[deriving(PartialEq,Show)]
pub struct Vertex {
    x: u8, // letter
    y: u8  // number
}

#[deriving(PartialEq,Show)]
pub enum Move {
    Stone(Vertex),
    Pass,
    Resign
}

#[allow(dead_code)]
#[deriving(PartialEq,Show)]
pub struct ColouredMove {
    pub player: Colour,
    pub move: Move
}

#[deriving(PartialEq,Show)]
pub enum StoneStatus {
    Alive,
    Seki,
    Dead
}

pub trait GoBot {
    // Static functions identifying the bot :

    // name (ex : "My super Bot")
    fn gtp_name(&self) -> String;
    // version (ex : "v2.3-r5")
    fn gtp_version(&self) -> String;

    // Any function returning a GTPError that it is not supposed
    // to return will be fatal to the framework.

    // Basic functions, must be implemented

    // clear_board : clears the board, can never fail
    fn gtp_clear_board(&mut self) -> ();
    // komi : sets the komi, can never fail, must accept absurd values
    fn gtp_komi(&mut self, komi: f32) -> ();
    // boardsize : sets the board size.
    // Returns InvalidBoardSize if the size is not supported.
    fn gtp_boardsize(&mut self, size: uint) -> Result<(), GTPError>;
    // play : plays the provided move on the board
    // Returns InvalidMove is the move is invalid
    fn gtp_play(&mut self, move: ColouredMove) -> Result<(), GTPError>;
    // genmove : ask the bot for a move of the chosen color
    // cannot fail, the bot must provide a move even if the last
    // played move is of the same colour
    // plays the move as well
    fn gtp_genmove(&mut self, player: Colour) -> Move;

    // Optional functions, if not iplemented, the corresponding
    // commands will not be activated
    // All these functions will be called once by the framework
    // at startup, then clear_board will be called

    // genmove_regression : like genmove, but must be deterministic
    // and must not actually play the move
    // should always return Ok(Move), never raise any error
    #[allow(unused_variable)]
    fn gtp_genmove_regression(&self, player: Colour) -> Result<Move, GTPError> {
        Err(NotImplemented)
    }
    // undo : undo last move if possible
    // if not, return Err(CannotUndo)
    // if undo is never possible, should not be implemented
    #[allow(unused_variable)]
    fn gtp_undo(&mut self) -> Result<(), GTPError> {
        Err(NotImplemented)
    }
    // place_free_handicap : The bot places its handicap stones
    // and returns a slice to a vector of Vertexes
    // it can place less stones if the asked number is too high
    // fails with Err(BoardNotEmpty) if board isn't empty
    #[allow(unused_variable)]
    fn gtp_place_free_handicap(&mut self, number: uint) -> Result<&[Vertex], GTPError> {
        Err(NotImplemented)
    }
    // set_free_handicap : uses the provided list as handicap stones
    // for black
    // fails with Err(BoardNotEmpty) if board isn't empty
    // fails woth Err(BadVertexList) if the vertex list is unusable
    // (two stones at the same place, or stones outside the board)
    #[allow(unused_variable)]
    fn gtp_set_free_handicap(&mut self, stones: &[Vertex]) -> Result<(), GTPError> {
        Err(NotImplemented)
    }
    // time_settings : sets the time settings for the game
    // it is only informative, the btot should count it's own time,
    // but the controller is supposed to enforce it
    // time are give in minute, should never fail
    #[allow(unused_variable)]
    fn gtp_time_settings(&mut self, main_time: int, byoyomi_time: int, byoyomi_stones: int) -> Result<(), GTPError> {
        Err(NotImplemented)
    }
    // final_status_list : returns a slice to the list of stones of
    // any color in the given status, in the opinion of the bot
    // should never fail
    #[allow(unused_variable)]
    fn gtp_final_status_list(&self, status: StoneStatus) -> Result<(), GTPError> {
        Err(NotImplemented)
    }
    // final_score : computes the bot's calculation of the final score
    // if it is a draw, float value must be 0 and colour is not important
    // should never fail
    #[allow(unused_variable)]
    fn gtp_final_score(&self) -> Result<(f32, Colour), GTPError> {
        Err(NotImplemented)
    }
    // showboard : returns a description of the board as saw by the bot :
    // (boardsize, black_stones, white_stones, black_captured_count, white_captured_count)
    // should never fail
    #[allow(unused_variable)]
    fn gtp_showboard(&self) -> Result<(int, &[Vertex], &[Vertex], int, int), GTPError> {
        Err(NotImplemented)
    }
}

// Vertex implementation for messing with strings
impl Vertex {
    #[allow(dead_code)]
    pub fn from_coords(x: u8, y:u8) -> Option<Vertex> {
        if x == 0 || x > 25 || y == 0 || y > 25 {
            None
        } else {
            Some(Vertex{x: x, y: y})
        }
    }

    #[allow(dead_code)]
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
        let number = ::std::u8::parse_bytes(text.as_bytes().slice_from(1), 10);
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

    #[allow(dead_code)]
    pub fn to_coords(&self) -> (u8, u8) {
        (self.x, self.y)
    }

    #[allow(dead_code)]
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
