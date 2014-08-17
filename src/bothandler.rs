use std::ascii::Ascii;
use std::string::String;
use api;
use parsing;
use boarddrawer;

// This struct is used to keep a record of which
// optional commands have been implemented by
// the bot and which haven't

static basic_command_list : &'static str = "protocol_version
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
        match bot.gtp_genmove_regression(api::Black) {
            Err(api::NotImplemented) => self.genmove_regression = false,
            _ => self.genmove_regression = true
        }
        match bot.gtp_undo() {
            Err(api::NotImplemented) => self.undo = false,
            _ => self.undo = true
        }
        match bot.gtp_fixed_handicap(1) {
            Err(api::NotImplemented) => self.fixed_handicap = false,
            _ => self.fixed_handicap = true
        }
        match bot.gtp_place_free_handicap(1) {
            Err(api::NotImplemented) => self.place_free_handicap = false,
            _ => self.place_free_handicap = true
        }
        match bot.gtp_set_free_handicap([api::Vertex::from_coords(2,2).unwrap()]) {
            Err(api::NotImplemented) => self.set_free_handicap = false,
            _ => self.set_free_handicap = true
        }
        match bot.gtp_time_settings(5, 0, 0) {
            Err(api::NotImplemented) => self.time_settings = false,
            _ => self.time_settings = true
        }
        match bot.gtp_final_status_list(api::Alive) {
            Err(api::NotImplemented) => self.final_status_list = false,
            _ => self.final_status_list = true
        }
        match bot.gtp_final_score() {
            Err(api::NotImplemented) => self.final_score = false,
            _ => self.final_score = true
        }
        match bot.gtp_showboard() {
            Err(api::NotImplemented) => self.showboard = false,
            _ => self.showboard = true
        }
        // lets reset the bot
        bot.gtp_clear_board();
    }

    // implementations of GTP commands

    fn cmd_list_commands(&self) -> String {
        let mut list = String::from_str(basic_command_list);
        if self.genmove_regression {
            //list = list.append("\nreg_genmove\nload_sgf");
            list = list.append("\nreg_genmove");
        }
        if self.undo {
            list = list.append("\nundo");
        }
        if self.place_free_handicap {
            list = list.append("\nplace_free_handicap");
        }
        if self.fixed_handicap {
            list = list.append("\nfixed_handicap");
        }
        if self.set_free_handicap {
            list = list.append("\nset_free_handicap");
        }
        if self.time_settings {
            list = list.append("\ntime_settings");
        }
        if self.final_status_list {
            list = list.append("\nfinal_status_list");
        }
        if self.final_score {
            list = list.append("\nfinal_score");
        }
        if self.showboard {
            list = list.append("\nshowboard");
        }
        list
    }

    fn cmd_known_command(&self, cmd: &[Ascii]) -> String {
        format!("{:b}", match cmd.as_str_ascii() {
            "protocol_version" | "name" | "version" |
            "known_command" | "list_commands" | "quit" |
            "boardsize" | "clear_board" | "komi" |
            "play" | "genmove" => true,
            // load_sgf will not be available in a first stage
            // "reg_genmove" | "load_sgf" => self.genmove_regression,
            "reg_genmove" => self.genmove_regression,
            "undo" => self.undo,
            "place_free_handicap" => self.place_free_handicap,
            "set_free_handiciap" | "fixed_handicap" => self.set_free_handicap,
            "time_settings" => self.time_settings,
            "final_status_list" => self.final_status_list,
            "final_score" => self.final_score,
            "showboard" => self.showboard,
            _ => false
        })
    }

    fn cmd_boardsize<T: api::GoBot>(&self, bot: &mut T, args: &[Ascii]) -> (bool, String) {
        match from_str::<uint>(args.as_str_ascii()) {
            Some(n) => match bot.gtp_boardsize(n) {
                Ok(()) => (true, String::new()),
                Err(api::InvalidBoardSize) => (false, String::from_str("invalid board size")),
                _ => fail!("Unexpected error in gtp_boardsize.")
            },
            None => (false, String::from_str("syntax error"))
        }
    }

    fn cmd_clear_board<T: api::GoBot>(&self, bot: &mut T) -> () {
        bot.gtp_clear_board();
    }

    fn cmd_komi<T: api::GoBot>(&self, bot: &mut T, args: &[Ascii]) -> (bool, String) {
        match from_str::<f32>(args.as_str_ascii()) {
            Some(k) => {bot.gtp_komi(k); (true, String::new())},
            None => (false, String::from_str("syntax error"))
        }
    }

    fn cmd_play<T: api::GoBot>(&self, bot: &mut T, args: &[Ascii]) -> (bool, String) {
        match parsing::parse_args(args, [parsing::ColouredMoveArg]) {
            Some(vect) => match vect[0] {
                parsing::ArgColouredMove(mv) => match bot.gtp_play(mv) {
                    Ok(()) => (true, String::new()),
                    Err(api::InvalidMove) => (false, String::from_str("invalid move")),
                    _ => fail!("Unexpected error in gtp_play.")
                },
                _ => unreachable!() // if parse_args returns a vector, it is valid
            },
            None => (false, String::from_str("syntax error"))
        }
    }

    fn cmd_genmove<T: api::GoBot>(&self, bot: &mut T, args: &[Ascii]) -> (bool, String) {
        match parsing::arg_parse_colour(args) {
            Some(col) => (true, bot.gtp_genmove(col).to_string()),
            None => (false, String::from_str("syntax error"))
        }
    }

    // optional functions, should not be called
    // if the bot does not implement their conterpart

    #[allow(unused_variable)]
    fn cmd_loadsgf<T: api::GoBot>(&self, bot: &mut T, args: &[Ascii]) -> (bool, String) {
        fail!("Not Implemented.");
    }

    fn cmd_reg_genmove<T: api::GoBot>(&self, bot: &mut T, args: &[Ascii]) -> (bool, String) {
        match parsing::arg_parse_colour(args) {
            Some(col) => match bot.gtp_genmove_regression(col) {
                Ok(mv) => (true, mv.to_string()),
                _ => fail!("Unexpected error in gtp_reg_genmove.")
            },
            None => (false, String::from_str("syntax error"))
        }
    }

    fn cmd_fixed_handicap<T: api::GoBot>(&self, bot: &mut T, args: &[Ascii]) -> (bool, String) {
        match from_str::<uint>(args.as_str_ascii()) {
            Some(n) if n >= 2 && n <= 9 => match bot.gtp_fixed_handicap(n) {
                Ok(vec) => (true, {
                    let mut it = vec.iter();
                    let mut out = it.next().to_string();
                    for &vrtx in it {
                        out = out.append(" ").append(vrtx.to_string().as_slice());
                    };
                    out }),
                Err(api::BoardNotEmpty) => (false, String::from_str("board not empty")),
                _ => fail!("Unexpected error in gtp_boardsize.")
            },
            Some(_) => (false, String::from_str("invalid number of stones")),
            None => (false, String::from_str("syntax error"))
        }
    }

    fn cmd_place_free_handicap<T: api::GoBot>(&self, bot: &mut T,  args: &[Ascii]) -> (bool, String) {
        match from_str::<uint>(args.as_str_ascii()) {
            Some(n) if n >= 2 => match bot.gtp_place_free_handicap(n) {
                Ok(vec) => (true, {
                    let mut it = vec.iter();
                    let mut out = it.next().to_string();
                    for &vrtx in it {
                        out = out.append(" ").append(vrtx.to_string().as_slice());
                    };
                    out }),
                Err(api::BoardNotEmpty) => (false, String::from_str("board not empty")),
                _ => fail!("Unexpected error in gtp_boardsize.")
            },
            Some(_) => (false, String::from_str("invalid number of stones")),
            None => (false, String::from_str("syntax error"))
        }
    }

    fn cmd_set_free_handicap<T: api::GoBot>(&self, bot: &mut T, args: &[Ascii]) -> (bool, String) {
        let mut lst: Vec<api::Vertex> = Vec::new();
        let mut it = args.split(|&c| { c == ' '.to_ascii()});
        for elem in it {
            match parsing::arg_parse_vertex(elem) {
                Some(vrtx) => { lst.push(vrtx); }
                _ => { return (false, String::from_str("syntax error")); }
            }
        }
        if lst.len() < 2 {
            return (false, String::from_str("bad vertex list"));
        }
        match bot.gtp_set_free_handicap(lst.as_slice()) {
            Ok(()) => (true, String::new()),
            Err(api::BadVertexList) => (false, String::from_str("bad vertex list")),
            Err(api::BoardNotEmpty) => (false, String::from_str("board not empty")),
            _ => fail!("Unexpected error in gtp_boardsize.")
        }
    }

    fn cmd_undo<T: api::GoBot>(&self, bot: &mut T) -> (bool, String) {
        match bot.gtp_undo() {
            Ok(()) => (true, String::new()),
            Err(api::CannotUndo) => (false, String::from_str("cannot undo")),
            _ => fail!("Unexpected error in gtp_undo.")
        }
    }

    fn cmd_time_settings<T: api::GoBot>(&self, bot: &mut T, args: &[Ascii]) -> (bool, String) {
        let mut it = args.splitn(3, |&c| { c == ' '.to_ascii()});
        match (it.next(), it.next(), it.next()) {
            (Some(a), Some(b), Some(c)) => match (from_str::<uint>(a.as_str_ascii()),
                                                  from_str::<uint>(b.as_str_ascii()),
                                                  from_str::<uint>(c.as_str_ascii())) {
                (Some(na), Some(nb), Some(nc)) => match bot.gtp_time_settings(na, nb, nc) {
                    Ok(()) => (true, String::new()),
                    Err(_) => fail!("Unexpected error in gtp_time_settings.")
                },
                _ => (false, String::from_str("syntax error"))
            },
            _ => (false, String::from_str("syntax error"))
        }
    }

    fn cmd_final_status_list<T: api::GoBot>(&self, bot: &mut T, args: &[Ascii]) -> (bool,String) {
        match parsing::arg_parse_stone_status(args) {
            Some(st) => match bot.gtp_final_status_list(st) {
                Ok(lst) => {
                    let mut output = String::new();
                    for &vrtx in lst.iter() {
                        output = output.append(vrtx.to_string().as_slice());
                    }
                    (true, output)
                },
                _ => fail!("Unexpected error in gtp_final_status_list.")
            },
            None => (false, String::from_str("syntax error"))
        }
    }

    fn cmd_final_score<T: api::GoBot>(&self, bot: &mut T) -> (bool, String) {
        match bot.gtp_final_score() {
            Ok(val) => match val {
                (0.0, _) => (true, String::from_str("0")),
                (x, api::White) => (true, format!("w+{}", x)),
                (x, api::Black) => (true, format!("b+{}", x))
                },
            Err(api::CannotScore) => (false, String::from_str("cannot score")),
            _ => fail!("Unexpected error in gtp_final_score.")
        }
    }

    fn cmd_showboard<T: api::GoBot>(&self, bot: &mut T) -> String {
        match bot.gtp_showboard(){
            Ok((bs, b_st, w_st, b_cp, w_cp)) => boarddrawer::draw_board(bs, b_st.as_slice(), w_st.as_slice(), b_cp, w_cp),
            _ => fail!("Unexpected error in gtp_showboard.")
        }
    }

    // dispatcher

    fn dispatch_cmd<T: api::GoBot>(&self, bot: &mut T, cmd: &[Ascii], args: &[Ascii]) -> (bool, String) {
        match cmd.as_str_ascii() {
            "protocol_version" => (true, String::from_str("2")),
            "name" => (true, bot.gtp_name()),
            "version" => (true, bot.gtp_version()),
            "known_command" => (true, self.cmd_known_command(args)),
            "list_commands" => (true, self.cmd_list_commands()),
            "boardsize" => self.cmd_boardsize(bot, args),
            "clear_board" => {self.cmd_clear_board(bot); (true, String::new())},
            "komi" => self.cmd_komi(bot, args),
            "play" => self.cmd_play(bot, args),
            "genmove" => self.cmd_genmove(bot, args),
            "loadsgf" => match self.genmove_regression {
                true => self.cmd_loadsgf(bot, args),
                false => (false, String::from_str("unknown command"))
            },
            "reg_genmove" => match self.genmove_regression {
                true => self.cmd_reg_genmove(bot, args),
                false => (false, String::from_str("unknown command"))
            },
            "fixed_handicap" => match self.set_free_handicap {
                true => self.cmd_fixed_handicap(bot, args),
                false => (false, String::from_str("unknown command"))
            },
            "set_free_handicap" => match self.set_free_handicap {
                true => self.cmd_set_free_handicap(bot, args),
                false => (false, String::from_str("unknown command"))
            },
            "place_free_handicap" => match self.place_free_handicap {
                true => self.cmd_place_free_handicap(bot, args),
                false => (false, String::from_str("unknown command"))
            },
            "undo" => match self.undo {
                true => self.cmd_undo(bot),
                false => (false, String::from_str("unknown command"))
            },
            "time_settings" => match self.time_settings {
                true => self.cmd_time_settings(bot, args),
                false => (false, String::from_str("unknown command"))
            },
            "time_left" => match self.genmove_regression {
                true => (true, String::new()), // noop for now
                false => (false, String::from_str("unknown command"))
            },
            "final_status_list" => match self.final_status_list {
                true => self.cmd_final_status_list(bot, args),
                false => (false, String::from_str("unknown command"))
            },
            "final_score" => match self.final_score {
                true => self.cmd_final_score(bot),
                false => (false, String::from_str("unknown command"))
            },
            "showboard" => match self.showboard {
                true => (true, self.cmd_showboard(bot)),
                false => (false, String::from_str("unknown command"))
            },
            _ => (false, String::from_str("unknown command"))
        }
    }
    // public functions

    pub fn from_bot<T: api::GoBot>(bot: &mut T) -> BotHandler {
        let mut handler = BotHandler::new();
        handler.populate(bot);
        handler
    }

    // handles content from ascii input
    // will parse and execute the first command encountered only
    // do nothing if no command is found
    pub fn handle_command<T: api::GoBot>(&self, bot: &mut T, input: &[Ascii]) -> (bool, String) {
        match parsing::parse_command(input) {
            Some(parsing::GTPCommand{id: id, command: command, args: args}) => {
                if command.as_slice().as_str_ascii() == "quit" {
                    (false, format!("={:s} bye",
                        match id {Some(i) => format!("{:u}", i), _ => String::new()}))
                } else {
                    let (result, output) = self.dispatch_cmd(bot, command.as_slice(), args.as_slice());
                    (true, format!("{:c}{:s} {:s}",
                        match result {true => '=', false => '?'},
                        match id {Some(i) => format!("{:u}", i), _ => String::new()},
                        output))
                    }
                },
            _ => {(true, String::new())}
        }
    }
}

