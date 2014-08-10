pub enum GTPError {
    NotImplemented,
    InvalidBoardSize,
    InvalidMove,
    BadVertexList,
    BoardNotEmpty,
    CannotUndo,
}

pub enum Colour {
    Black,
    White
}

#[allow(dead_code)]
pub struct Vertex {
    x: int, // letter
    y: int  // number
}

pub enum Move {
    Stone(Vertex),
    Pass,
    Resign
}

#[allow(dead_code)]
pub struct ColouredMove {
    player: Colour,
    move: Move
}

pub enum StoneStatus {
    Alive,
    Seki,
    Dead
}

pub trait GoBot {
    // Static functions identifying the bot :

    // name (ex : "My super Bot")
    fn gtp_name() -> &'static str;
    // version (ex : "v2.3-r5")
    fn gtp_version() -> &'static str;

    // Any function returning a GTPError that it is not supposed
    // to return will be fatal to the framework.

    // Basic functions, must be implemented

    // clear_board : clears the board, can never fail
    fn gtp_clear_board(&self) -> ();
    // komi : sets the komi, can never fail, must accept absurd values
    fn gtp_komi(&self, komi: f32) -> ();
    // boardsize : sets the board size.
    // Returns InvalidBoardSize if the size is not supported.
    fn gtp_boardsize(&self, size: uint) -> Result<(), GTPError>;
    // play : plays the provided move on the board
    // Returns InvalidMove is the move is invalid
    fn gtp_play(&self, move: ColouredMove) -> Result<(), GTPError>;
    // genmove : ask the bot for a move of the chosen color
    // cannot fail, the bot must provide a move even if the last
    // played move is of the same colour
    fn gtp_genmove(&self, player: Colour) -> Move;

    // Optional functions, if not iplemented, the corresponding
    // commands will not be activated
    // All these functions will be called once by the framework
    // at startup, then clear_board will be called

    // genmove_determinisic : like genmove, but must be deterministic
    // if genmove is already deterministic, can be aliased to it
    // should always return Ok(Move), never raise any error
    #[allow(unused_variable)]
    fn gtp_genmove_deterministic(&self, player: Colour) -> Result<Move, GTPError> {
        Err(NotImplemented)
    }
    // undo : undo last move if possible
    // if not, return Err(CannotUndo)
    // if undo is never possible, should not be implemented
    #[allow(unused_variable)]
    fn gtp_undo(&self) -> Result<(), GTPError> {
        Err(NotImplemented)
    }
    // place_free_handicap : The bot places its handicap stones
    // and returns a slice to a vector of Vertexes
    // it can place less stones if the asked number is too high
    // fails with Err(BoardNotEmpty) if board isn't empty
    #[allow(unused_variable)]
    fn gtp_place_free_handicap(&self, number: uint) -> Result<&[Vertex], GTPError> {
        Err(NotImplemented)
    }
    // set_free_handicap : uses the provided list as handicap stones
    // for black
    // fails with Err(BoardNotEmpty) if board isn't empty
    // fails woth Err(BadVertexList) if the vertex list is unusable
    // (two stones at the same place, or stones outside the board)
    #[allow(unused_variable)]
    fn gtp_set_free_handicap(&self, stones: &[Vertex]) -> Result<(), GTPError> {
        Err(NotImplemented)
    }
    // time_settings : sets the time settings for the game
    // it is only informative, the btot should count it's own time,
    // but the controller is supposed to enforce it
    // time are give in minute, should never fail
    #[allow(unused_variable)]
    fn gtp_time_settings(&self, main_time: int, byoyomi_time: int, byoyomi_stones: int) -> Result<(), GTPError> {
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
