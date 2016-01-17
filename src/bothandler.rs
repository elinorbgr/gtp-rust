use std::str::FromStr;
use std::string::String;
use std::usize;
use api;
use parsing;
use boarddrawer;

// This struct is used to keep a record of which
// optional commands have been implemented by
// the bot and which haven't

static BASIC_COMMAND_LIST : &'static str = "protocol_version
name
version
known_command
list_commands
quit
boardsize
clear_board
komi
play
genmove";

pub struct BotHandler {
    genmove_regression: bool,
    undo: bool,
    fixed_handicap: bool,
    place_free_handicap: bool,
    set_free_handicap: bool,
    time_settings: bool,
    final_status_list: bool,
    final_score: bool,
    showboard: bool
}

impl BotHandler {
    fn new() -> BotHandler{
        BotHandler{
            genmove_regression: false,
            undo: false,
            fixed_handicap: false,
            place_free_handicap: false,
            set_free_handicap: false,
            time_settings: false,
            final_status_list: false,
            final_score: false,
            showboard: false
        }
    }

    fn populate<T: api::GoBot>(&mut self, bot: &mut T) {
        match bot.gtp_genmove_regression(api::Colour::Black) {
            Err(api::GTPError::NotImplemented) => self.genmove_regression = false,
            _ => self.genmove_regression = true
        }
        match bot.gtp_undo() {
            Err(api::GTPError::NotImplemented) => self.undo = false,
            _ => self.undo = true
        }
        match bot.gtp_fixed_handicap(1) {
            Err(api::GTPError::NotImplemented) => self.fixed_handicap = false,
            _ => self.fixed_handicap = true
        }
        match bot.gtp_place_free_handicap(1) {
            Err(api::GTPError::NotImplemented) => self.place_free_handicap = false,
            _ => self.place_free_handicap = true
        }
        match bot.gtp_set_free_handicap(&[api::Vertex::from_coords(2,2).unwrap()]) {
            Err(api::GTPError::NotImplemented) => self.set_free_handicap = false,
            _ => self.set_free_handicap = true
        }
        match bot.gtp_time_settings(5, 0, 0) {
            Err(api::GTPError::NotImplemented) => self.time_settings = false,
            _ => self.time_settings = true
        }
        match bot.gtp_final_status_list(api::StoneStatus::Alive) {
            Err(api::GTPError::NotImplemented) => self.final_status_list = false,
            _ => self.final_status_list = true
        }
        match bot.gtp_final_score() {
            Err(api::GTPError::NotImplemented) => self.final_score = false,
            _ => self.final_score = true
        }
        match bot.gtp_showboard() {
            Err(api::GTPError::NotImplemented) => self.showboard = false,
            _ => self.showboard = true
        }
        // lets reset the bot
        bot.gtp_clear_board();
    }

    // implementations of GTP commands

    fn cmd_list_commands<T: api::GoBot>(&self, bot: &T) -> String {
        let mut list = BASIC_COMMAND_LIST.to_string();
        if self.genmove_regression {
            list.push_str("\nreg_genmove\nloadsgf");
        }
        if self.undo {
            list.push_str("\nundo");
        }
        if self.place_free_handicap {
            list.push_str("\nplace_free_handicap");
        }
        if self.fixed_handicap {
            list.push_str("\nfixed_handicap");
        }
        if self.set_free_handicap {
            list.push_str("\nset_free_handicap");
        }
        if self.time_settings {
            list.push_str("\ntime_settings");
        }
        if self.final_status_list {
            list.push_str("\nfinal_status_list");
        }
        if self.final_score {
            list.push_str("\nfinal_score");
        }
        if self.showboard {
            list.push_str("\nshowboard");
        }
        for cmd in bot.gtp_list_custom_commands().iter() {
            list.push_str(&format!("\n{}", cmd));
        }
        list
    }

    fn cmd_known_command<T: api::GoBot>(&self, bot: &T, cmd: &str) -> String {
        format!("{}", match cmd {
            "protocol_version" | "name" | "version" |
            "known_command" | "list_commands" | "quit" |
            "boardsize" | "clear_board" | "komi" |
            "play" | "genmove" => true,
            "reg_genmove" | "loadsgf" => self.genmove_regression,
            "undo" => self.undo,
            "place_free_handicap" => self.place_free_handicap,
            "set_free_handiciap" | "fixed_handicap" => self.set_free_handicap,
            "time_settings" => self.time_settings,
            "final_status_list" => self.final_status_list,
            "final_score" => self.final_score,
            "showboard" => self.showboard,
            _ => bot.gtp_known_custom_command(cmd)
        })
    }

    fn cmd_boardsize<T: api::GoBot>(&self, bot: &mut T, args: &str) -> (bool, String) {
        match usize::from_str(args) {
            Ok(n) => match bot.gtp_boardsize(n) {
                Ok(()) => (true, String::new()),
                Err(api::GTPError::InvalidBoardSize) => (false, "invalid board size".to_string()),
                _ => panic!("Unexpected error in gtp_boardsize.")
            },
            Err(_) => (false, "syntax error".to_string())
        }
    }

    fn cmd_clear_board<T: api::GoBot>(&self, bot: &mut T) -> () {
        bot.gtp_clear_board();
    }

    fn cmd_komi<T: api::GoBot>(&self, bot: &mut T, args: &str) -> (bool, String) {
        match f32::from_str(args) {
            Ok(k) => {bot.gtp_komi(k); (true, String::new())},
            Err(_) => (false, "syntax error".to_string()),
        }
    }

    fn cmd_play<T: api::GoBot>(&self, bot: &mut T, args: &str) -> (bool, String) {
        match parsing::parse_args(args, &[parsing::ArgType::ColouredMoveArg]) {
            Some(vect) => match vect[0] {
                parsing::Argument::ArgColouredMove(mv) => match bot.gtp_play(mv) {
                    Ok(()) => (true, String::new()),
                    Err(api::GTPError::InvalidMove) => (false, "invalid move".to_string()),
                    _ => panic!("Unexpected error in gtp_play.")
                },
                _ => unreachable!() // if parse_args returns a vector, it is valid
            },
            None => (false, "syntax error".to_string()),
        }
    }

    fn cmd_genmove<T: api::GoBot>(&self, bot: &mut T, args: &str) -> (bool, String) {
        match parsing::arg_parse_colour(args) {
            Some(col) => (true, bot.gtp_genmove(col).to_string()),
            None => (false, "syntax error".to_string()),
        }
    }

    // optional functions, should not be called
    // if the bot does not implement their conterpart

    #[allow(unused_variables)]
    fn cmd_loadsgf<T: api::GoBot>(&self, bot: &mut T, args: &str) -> (bool, String) {
        let error = (false, "syntax error".to_string());
        let mut args_iter = args.splitn(3, ' ');
        match args_iter.next() {
            Some(filename) => {
                let number = match args_iter.next() {
                    Some(t) => {
                        match usize::from_str(t) {
                            Ok(n) => n,
                            Err(_) => { return error },
                        }
                    }
                    None => usize::MAX
                };
                match bot.gtp_loadsgf(filename, number) {
                    Ok(_) => (true, String::new()),
                    Err(_) => error,
                }
            }
            None => error,
        }
    }

    fn cmd_reg_genmove<T: api::GoBot>(&self, bot: &mut T, args: &str) -> (bool, String) {
        match parsing::arg_parse_colour(args) {
            Some(col) => match bot.gtp_genmove_regression(col) {
                Ok(mv) => (true, mv.to_string()),
                _ => panic!("Unexpected error in gtp_reg_genmove.")
            },
            None => (false, "syntax error".to_string())
        }
    }

    fn cmd_fixed_handicap<T: api::GoBot>(&self, bot: &mut T, args: &str) -> (bool, String) {
        match usize::from_str(args) {
            Ok(n) if n >= 2 && n <= 9 => match bot.gtp_fixed_handicap(n) {
                Ok(vec) => (true, {
                    let mut it = vec.iter();
                    let mut out = it.next().unwrap().to_string();
                    for vrtx in it {
                        out.push_str(" ");
                        out.push_str(&vrtx.to_string());
                    };
                    out }),
                Err(api::GTPError::BoardNotEmpty) => (false, "board not empty".to_string()),
                _ => panic!("Unexpected error in gtp_boardsize.")
            },
            Ok(_) => (false, "invalid number of stones".to_string()),
            Err(_) => (false, "syntax error".to_string())
        }
    }

    fn cmd_place_free_handicap<T: api::GoBot>(&self, bot: &mut T,  args: &str) -> (bool, String) {
        match usize::from_str(args) {
            Ok(n) if n >= 2 => match bot.gtp_place_free_handicap(n) {
                Ok(vec) => (true, {
                    let mut it = vec.iter();
                    let mut out = it.next().unwrap().to_string();
                    for vrtx in it {
                        out.push_str(" ");
                        out.push_str(&vrtx.to_string());
                    };
                    out }),
                Err(api::GTPError::BoardNotEmpty) => (false, "board not empty".to_string()),
                _ => panic!("Unexpected error in gtp_boardsize.")
            },
            Ok(_) => (false, "invalid number of stones".to_string()),
            Err(_) => (false, "syntax error".to_string()),
        }
    }

    fn cmd_set_free_handicap<T: api::GoBot>(&self, bot: &mut T, args: &str) -> (bool, String) {
        let mut lst: Vec<api::Vertex> = Vec::new();
        let it = args.split(' ');
        for elem in it {
            match parsing::arg_parse_vertex(elem) {
                Some(vrtx) => { lst.push(vrtx); }
                _ => { return (false, "syntax error".to_string()); }
            }
        }
        if lst.len() < 2 {
            return (false, "bad vertex list".to_string());
        }
        match bot.gtp_set_free_handicap(&lst) {
            Ok(()) => (true, String::new()),
            Err(api::GTPError::BadVertexList) => (false, "bad vertex list".to_string()),
            Err(api::GTPError::BoardNotEmpty) => (false, "board not empty".to_string()),
            _ => panic!("Unexpected error in gtp_boardsize.")
        }
    }

    fn cmd_undo<T: api::GoBot>(&self, bot: &mut T) -> (bool, String) {
        match bot.gtp_undo() {
            Ok(()) => (true, String::new()),
            Err(api::GTPError::CannotUndo) => (false, "cannot undo".to_string()),
            _ => panic!("Unexpected error in gtp_undo.")
        }
    }

    fn cmd_time_settings<T: api::GoBot>(&self, bot: &mut T, args: &str) -> (bool, String) {
        let mut it = args.splitn(4, ' ');
        match (it.next(), it.next(), it.next()) {
            (Some(a), Some(b), Some(c)) => match (usize::from_str(a),
                                                  usize::from_str(b),
                                                  usize::from_str(c)) {
                (Ok(na), Ok(nb), Ok(nc)) => match bot.gtp_time_settings(na, nb, nc) {
                    Ok(()) => (true, String::new()),
                    Err(_) => panic!("Unexpected error in gtp_time_settings.")
                },
                _ => (false, "syntax error".to_string()),
            },
            _ => (false, "syntax error".to_string()),
        }
    }

    fn cmd_final_status_list<T: api::GoBot>(&self, bot: &mut T, args: &str) -> (bool,String) {
        match parsing::arg_parse_stone_status(args) {
            Some(st) => match bot.gtp_final_status_list(st) {
                Ok(lst) => {
                    let mut output = String::new();
                    for vrtx in lst.iter() {
                        output.push_str(&vrtx.to_string());
                    }
                    (true, output)
                },
                _ => panic!("Unexpected error in gtp_final_status_list.")
            },
            None => (false, "syntax error".to_string())
        }
    }

    fn cmd_final_score<T: api::GoBot>(&self, bot: &mut T) -> (bool, String) {
        match bot.gtp_final_score() {
            Ok(val) => match val {
                (0.0, _) => (true, "0".to_string()),
                (x, api::Colour::White) => (true, format!("w+{}", x)),
                (x, api::Colour::Black) => (true, format!("b+{}", x))
                },
            Err(api::GTPError::CannotScore) => (false, "cannot score".to_string()),
            _ => panic!("Unexpected error in gtp_final_score.")
        }
    }

    fn cmd_showboard<T: api::GoBot>(&self, bot: &mut T) -> String {
        match bot.gtp_showboard(){
            Ok((bs, b_st, w_st, b_cp, w_cp)) => boarddrawer::draw_board(bs, &b_st, &w_st, b_cp, w_cp),
            _ => panic!("Unexpected error in gtp_showboard.")
        }
    }

    // dispatcher

    fn dispatch_cmd<T: api::GoBot>(&self, bot: &mut T, cmd: &str, args: &str) -> (bool, String) {
        match cmd {
            "protocol_version" => (true, "2".to_string()),
            "name" => (true, bot.gtp_name()),
            "version" => (true, bot.gtp_version()),
            "known_command" => (true, self.cmd_known_command(bot, args)),
            "list_commands" => (true, self.cmd_list_commands(bot)),
            "boardsize" => self.cmd_boardsize(bot, args),
            "clear_board" => {self.cmd_clear_board(bot); (true, String::new())},
            "komi" => self.cmd_komi(bot, args),
            "play" => self.cmd_play(bot, args),
            "genmove" => self.cmd_genmove(bot, args),
            "loadsgf" => match self.genmove_regression {
                true => self.cmd_loadsgf(bot, args),
                false => (false, "unknown command".to_string())
            },
            "reg_genmove" => match self.genmove_regression {
                true => self.cmd_reg_genmove(bot, args),
                false => (false, "unknown command".to_string())
            },
            "fixed_handicap" => match self.set_free_handicap {
                true => self.cmd_fixed_handicap(bot, args),
                false => (false, "unknown command".to_string())
            },
            "set_free_handicap" => match self.set_free_handicap {
                true => self.cmd_set_free_handicap(bot, args),
                false => (false, "unknown command".to_string())
            },
            "place_free_handicap" => match self.place_free_handicap {
                true => self.cmd_place_free_handicap(bot, args),
                false => (false, "unknown command".to_string())
            },
            "undo" => match self.undo {
                true => self.cmd_undo(bot),
                false => (false, "unknown command".to_string())
            },
            "time_settings" => match self.time_settings {
                true => self.cmd_time_settings(bot, args),
                false => (false, "unknown command".to_string())
            },
            "time_left" => match self.genmove_regression {
                true => (true, String::new()), // noop for now
                false => (false, "unknown command".to_string())
            },
            "final_status_list" => match self.final_status_list {
                true => self.cmd_final_status_list(bot, args),
                false => (false, "unknown command".to_string())
            },
            "final_score" => match self.final_score {
                true => self.cmd_final_score(bot),
                false => (false, "unknown command".to_string())
            },
            "showboard" => match self.showboard {
                true => (true, self.cmd_showboard(bot)),
                false => (false, "unknown command".to_string())
            },
            _ => bot.gtp_custom_command(cmd, args)
        }
    }
    // public functions

    pub fn from_bot<T: api::GoBot>(bot: &mut T) -> BotHandler {
        let mut handler = BotHandler::new();
        handler.populate(bot);
        handler
    }

    // handles content from AsciiExt input
    // will parse and execute the first command encountered only
    // do nothing if no command is found
    pub fn handle_command<T: api::GoBot>(&self, bot: &mut T, input: &str) -> (bool, String) {
        match parsing::parse_command(input) {
            Some(parsing::GTPCommand{id, command, args}) => {
                if command == "quit" {
                    (false, format!("={} bye",
                        match id {Some(i) => format!("{}", i), _ => String::new()}))
                } else {
                    let (result, output) = self.dispatch_cmd(bot, &command, &args);
                    (true, format!("{}{} {}",
                        match result {true => '=', false => '?'},
                        match id {Some(i) => format!("{}", i), _ => String::new()},
                        output))
                    }
                },
            _ => {(true, String::new())}
        }
    }
}
