use std::vec::Vec;
use std::ascii::Ascii;
use api;

// Strips all ignored content from input string
// according to specifications of GTPv2
fn strip_input(input: &[Ascii]) -> Vec<Ascii> {
    let mut output: Vec<Ascii> = Vec::new();
    let mut last_char: Ascii = '\n'.to_ascii(); // set initial lastchar as LF
    let mut in_comment = false;
    for &c in input.iter() {
        if c == '\n'.to_ascii() { // newline
            in_comment = false;
            if last_char == '\n'.to_ascii() {
                // ignore multiple newlines
                continue;
            }
            last_char = '\n'.to_ascii();
            output.push('\n'.to_ascii());
            continue;
        }
        if c == '#'.to_ascii() {
            in_comment = true;
            continue;
        }
        if in_comment {
            // we are in a comment, and the caracter did not end it,
            continue;
        }
        if c.is_blank() { // space or \t
            if last_char.to_char().is_whitespace() {
                // multiple spaces are discarded
                // as well as spaces in the begining of a line
                continue;
            }
            last_char = ' '.to_ascii();
            output.push(' '.to_ascii());
            continue;
        }
        if c.is_control() {
            // non printable, drop it
            continue;
        }
        // any other character is kept as it
        last_char = c;
        output.push(c);
    }
    output
}

#[deriving(PartialEq, Show)]
pub struct GTPCommand {
    pub id: Option<u32>,
    pub command: Vec<Ascii>,
    pub args: Vec<Ascii>
}

// parses a command from a line
// input is suposed to be a single stripped line
// without trailing \n
fn parse_command_from_stripped(line: &[Ascii]) -> Option<GTPCommand> {
    if line.as_str_ascii().is_whitespace() {
        // empty line no command to parse
        None
    } else {
        let mut first_split = line.splitn(1, |&c| { c == ' '.to_ascii()});
        // there is always a first value
        let first_part = first_split.next().unwrap();
        let mut to_split = match first_split.next() {
            Some(text) => text,
            _ => "".to_ascii()
        };
        let id = from_str::<u32>(first_part.as_str_ascii());
        match id {
            None => { to_split = line; },
            _ => { }
        };
        let mut second_split = to_split.splitn(1, |&c| { c == ' '.to_ascii()});
        match second_split.next() {
            Some(text) if text.len() > 0 => Some(GTPCommand{
                id: id,
                command: Vec::from_slice(text),
                args: match second_split.next() {
                    Some(arguments) => Vec::from_slice(arguments),
                    _ => Vec::new()
                }
            }),
            _ => None
        }
    }
}

// parses a command from a un-stripped line
// if inputed several lines, only the first non empty
// and non comment is parsed
pub fn parse_command(input: &[Ascii]) -> Option<GTPCommand> {
    match strip_input(input).as_slice().splitn(1, |&c| {c == '\n'.to_ascii()}).next() {
        Some(line) => parse_command_from_stripped(line),
        _ => None
    }
}

#[deriving(PartialEq, Show)]
pub enum ArgType {
    ColourArg,
    VertexArg,
    MoveArg,
    ColouredMoveArg,
    StoneStatusArg
}

#[deriving(PartialEq, Show)]
pub enum Argument {
    ArgColour(api::Colour),
    ArgVertex(api::Vertex),
    ArgMove(api::Move),
    ArgColouredMove(api::ColouredMove),
    ArgStoneStatus(api::StoneStatus)
}

fn arg_parse_colour (input: &[Ascii]) -> Option<api::Colour> {
    match input.to_lower().as_slice().as_str_ascii() {
        "w" | "white" => Some(api::White),
        "b" | "black" => Some(api::Black),
        _ => None
    }
}

fn arg_parse_vertex (input: &[Ascii]) -> Option<api::Vertex> {
    api::Vertex::from_str(input.as_str_ascii())
}

fn arg_parse_move (input: &[Ascii]) -> Option<api::Move> {
    match input.to_lower().as_slice().as_str_ascii() {
        "pass" => Some(api::Pass),
        "resign" => Some(api::Resign),
        _ => match arg_parse_vertex(input) {
            Some(v) => Some(api::Stone(v)),
            _ => None
        }
    }
}

fn arg_parse_stone_status (input: &[Ascii]) -> Option<api::StoneStatus> {
    match input.to_lower().as_slice().as_str_ascii() {
        "alive" => Some(api::Alive),
        "dead" => Some(api::Dead),
        "seki" => Some(api::Seki),
        _ => None
    }
}

pub fn parse_args (input: &[Ascii], types: &[ArgType]) -> Option<Vec<Argument>> {
    let mut args_iter = input.split(|&c| {c == ' '.to_ascii() });
    let mut vect: Vec<Argument> = Vec::new();
    for &arg_type in types.iter() {
        let itered = args_iter.next();
        if itered == None {
            return None;
        }
        let input = itered.unwrap();
        match match arg_type {
            ColourArg => match arg_parse_colour(input) {
                Some(col) => Some(ArgColour(col)),
                _ => None
            },
            VertexArg => match arg_parse_vertex(input) {
                Some(vrtx) => Some(ArgVertex(vrtx)),
                _ => None
            },
            MoveArg => match arg_parse_move(input) {
                Some(mv) => Some(ArgMove(mv)),
                _ => None
            },
            StoneStatusArg => match arg_parse_stone_status(input) {
                Some(st) => Some(ArgStoneStatus(st)),
                _ => None
            },
            ColouredMoveArg => {
                let itered2 = args_iter.next();
                if itered2 == None {
                    return None;
                }
                let input2 = itered2.unwrap();
                match (arg_parse_colour(input), arg_parse_move(input2)) {
                    (Some(col), Some(mv)) => Some(ArgColouredMove(api::ColouredMove{player: col, move: mv})),
                    _ => None
                }
            }
        } { // match
            Some(arg) => { vect.push(arg); },
            None => { return None; }
        }
    }
    Some(vect)
}

#[cfg(test)]
mod tests {
    use api;
    #[test]
    fn strip_input() {
        let input = "command1 \t and \x15 argu\x07ments\n# this is a comment\n  command2    !op # comment\n\n\nfoo bar\n\n".to_ascii();
        let expected_output = "command1 and arguments\ncommand2 !op \nfoo bar\n".to_ascii();
        let output = super::strip_input(input);
        assert_eq!(output.as_slice(), expected_output);
    }

    #[test]
    fn parse_command_from_stripped() {
        assert_eq!(super::parse_command_from_stripped("".to_ascii()), None);
        assert_eq!(super::parse_command_from_stripped("56".to_ascii()), None);
        assert_eq!(super::parse_command_from_stripped("foo".to_ascii()), Some(
            super::GTPCommand{
                id: None,
                command: Vec::from_slice("foo".to_ascii()),
                args: Vec::new()
            }));
        assert_eq!(super::parse_command_from_stripped("foo bar baz".to_ascii()), Some(
            super::GTPCommand{
                id: None,
                command: Vec::from_slice("foo".to_ascii()),
                args: Vec::from_slice("bar baz".to_ascii())
            }));
        assert_eq!(super::parse_command_from_stripped("42 foo".to_ascii()), Some(
            super::GTPCommand{
                id: Some(42u32),
                command: Vec::from_slice("foo".to_ascii()),
                args: Vec::from_slice("".to_ascii())
            }));
        assert_eq!(super::parse_command_from_stripped("42 foo bar baz".to_ascii()), Some(
            super::GTPCommand{
                id: Some(42u32),
                command: Vec::from_slice("foo".to_ascii()),
                args: Vec::from_slice("bar baz".to_ascii())
            }));
    }

    #[test]
    fn parse_command() {
        assert_eq!(
            super::parse_command("  #  this is a comment\n\t  \n  # this as well".to_ascii()),
            None
        );
        assert_eq!(
            super::parse_command("#this command is really cool\n42 foo Cake is a lie  # cool isn't it ?".to_ascii()),
            Some(super::GTPCommand{
                id: Some(42u32),
                command: Vec::from_slice("foo".to_ascii()),
                args: Vec::from_slice("Cake is a lie ".to_ascii())
            })
        );
    }

    #[test]
    fn arg_parse_colour() {
        assert_eq!(super::arg_parse_colour("BlAcK".to_ascii()), Some(api::Black));
        assert_eq!(super::arg_parse_colour("b".to_ascii()), Some(api::Black));
        assert_eq!(super::arg_parse_colour("WHIte".to_ascii()), Some(api::White));
        assert_eq!(super::arg_parse_colour("W".to_ascii()), Some(api::White));
        assert_eq!(super::arg_parse_colour("FOO".to_ascii()), None);
        assert_eq!(super::arg_parse_colour("bar".to_ascii()), None);
    }

    #[test]
    fn arg_parse_move() {
        assert_eq!(super::arg_parse_move("ReSiGn".to_ascii()), Some(api::Resign));
        assert_eq!(super::arg_parse_move("PasS".to_ascii()), Some(api::Pass));
        assert_eq!(super::arg_parse_move("A12".to_ascii()), Some(api::Stone(api::Vertex::from_coords(1,12).unwrap())));
        assert_eq!(super::arg_parse_move("T7".to_ascii()), Some(api::Stone(api::Vertex::from_coords(19,7).unwrap())));
        assert_eq!(super::arg_parse_move("F26".to_ascii()), None);
        assert_eq!(super::arg_parse_move("I13".to_ascii()), None);
        assert_eq!(super::arg_parse_move("foo".to_ascii()), None);

    }

    #[test]
    fn parse_args() {
        let arg_string = "W G7 alive black pass E5".to_ascii();
        let arg_types = vec!(
            super::ColouredMoveArg,
            super::StoneStatusArg,
            super::ColourArg,
            super::MoveArg,
            super::VertexArg
        );
        let expected_args = vec!(
            super::ArgColouredMove(api::ColouredMove{player: api::White, move: api::Stone(api::Vertex::from_coords(7,7).unwrap())}),
            super::ArgStoneStatus(api::Alive),
            super::ArgColour(api::Black),
            super::ArgMove(api::Pass),
            super::ArgVertex(api::Vertex::from_coords(5,5).unwrap())
        );
        let parsed_args = super::parse_args(arg_string, arg_types.as_slice());
        assert_eq!(parsed_args.unwrap(), expected_args);
    }
}
