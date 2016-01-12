use std::vec::Vec;
use std::str::FromStr;
use api;

// Strips all ignored content from input string
// according to specifications of GTPv2
fn strip_input(input: &str) -> String {
    let mut output = String::new();
    let mut last_char: char = '\n'; // set initial lastchar as LF
    let mut in_comment = false;
    for c in input.chars() {
        if c == '\n' { // newline
            in_comment = false;
            if last_char == '\n' {
                // ignore multiple newlines
                continue;
            }
            last_char = '\n';
            output.push('\n');
            continue;
        }
        if c == '#' {
            in_comment = true;
            continue;
        }
        if in_comment {
            // we are in a comment, and the caracter did not end it,
            continue;
        }
        if c == ' ' || c == '\t' { // space or \t
            if last_char.is_whitespace() {
                // multiple spaces are discarded
                // as well as spaces in the begining of a line
                continue;
            }
            last_char = ' ';
            output.push(' ');
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

#[derive(PartialEq, Debug)]
pub struct GTPCommand {
    pub id: Option<u32>,
    pub command: String,
    pub args: String,
}

// parses a command from a line
// input is suposed to be a single stripped line
// without trailing \n
fn parse_command_from_stripped(line: &str) -> Option<GTPCommand> {
    if line.chars().all(|c| c.is_whitespace()) {
        // empty line no command to parse
        None
    } else {
        let mut first_split = line.splitn(2, ' ');
        // there is always a first value
        let first_part = first_split.next().unwrap();
        let mut to_split = match first_split.next() {
            Some(text) => text,
            _ => "",
        };
        println!("line: {}", line);
        println!("first_part: {}", first_part);
        let id = u32::from_str(first_part);
        println!("id: {}", id.is_ok());
        match id {
            Err(_) => { to_split = line; },
            _ => { }
        };
        let mut second_split = to_split.splitn(2, ' ');
        match second_split.next() {
            Some(text) if text.len() > 0 => Some(GTPCommand {
                id: id.ok(),
                command: text.to_string(),
                args: match second_split.next() {
                    Some(arguments) => arguments.to_string(),
                    _ => String::new()
                }
            }),
            _ => None
        }
    }
}

// parses a command from a un-stripped line
// if inputed several lines, only the first non empty
// and non comment is parsed
pub fn parse_command(input: &str) -> Option<GTPCommand> {
    match strip_input(input).splitn(1, '\n').next() {
        Some(line) => parse_command_from_stripped(line),
        _ => None
    }
}

#[derive(PartialEq, Debug)]
pub enum ArgType {
    ColourArg,
    VertexArg,
    MoveArg,
    ColouredMoveArg,
    StoneStatusArg
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Argument {
    ArgColour(api::Colour),
    ArgVertex(api::Vertex),
    ArgMove(api::Move),
    ArgColouredMove(api::ColouredMove),
    ArgStoneStatus(api::StoneStatus)
}

pub fn arg_parse_colour (input: &str) -> Option<api::Colour> {
    match input.to_lowercase().as_ref() {
        "w" | "white" => Some(api::Colour::White),
        "b" | "black" => Some(api::Colour::Black),
        _ => None
    }
}

pub fn arg_parse_vertex (input: &str) -> Option<api::Vertex> {
    api::Vertex::from_str(input)
}

pub fn arg_parse_move (input: &str) -> Option<api::Move> {
    match input.to_lowercase().as_ref() {
        "pass" => Some(api::Move::Pass),
        "resign" => Some(api::Move::Resign),
        _ => match arg_parse_vertex(input) {
            Some(v) => Some(api::Move::Stone(v)),
            _ => None
        }
    }
}

pub fn arg_parse_stone_status (input: &str) -> Option<api::StoneStatus> {
    match input.to_lowercase().as_ref() {
        "alive" => Some(api::StoneStatus::Alive),
        "dead" => Some(api::StoneStatus::Dead),
        "seki" => Some(api::StoneStatus::Seki),
        _ => None
    }
}

pub fn parse_args (input: &str, types: &[ArgType]) -> Option<Vec<Argument>> {
    let mut args_iter = input.split(' ');
    let mut vect: Vec<Argument> = Vec::new();
    for arg_type in types {
        let itered = args_iter.next();
        if itered == None {
            return None;
        }
        let input = itered.unwrap();
        match match *arg_type {
            ArgType::ColourArg => match arg_parse_colour(input) {
                Some(col) => Some(Argument::ArgColour(col)),
                _ => None
            },
            ArgType::VertexArg => match arg_parse_vertex(input) {
                Some(vrtx) => Some(Argument::ArgVertex(vrtx)),
                _ => None
            },
            ArgType::MoveArg => match arg_parse_move(input) {
                Some(mv) => Some(Argument::ArgMove(mv)),
                _ => None
            },
            ArgType::StoneStatusArg => match arg_parse_stone_status(input) {
                Some(st) => Some(Argument::ArgStoneStatus(st)),
                _ => None
            },
            ArgType::ColouredMoveArg => {
                let itered2 = args_iter.next();
                if itered2 == None {
                    return None;
                }
                let input2 = itered2.unwrap();
                match (arg_parse_colour(input), arg_parse_move(input2)) {
                    (Some(col), Some(mv)) => Some(Argument::ArgColouredMove(api::ColouredMove{player: col, mov: mv})),
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
        let input = "command1 \t and \x15 argu\x07ments\n# this is a comment\n  command2    !op # comment\n\n\nfoo bar\n\n";
        let expected_output = "command1 and arguments\ncommand2 !op \nfoo bar\n";
        let output = super::strip_input(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn parse_command_from_stripped() {
        assert_eq!(super::parse_command_from_stripped(""), None);
        assert_eq!(super::parse_command_from_stripped("56"), None);
        assert_eq!(super::parse_command_from_stripped("foo"), Some(
            super::GTPCommand{
                id: None,
                command: "foo".to_string(),
                args: String::new(),
            }));
        assert_eq!(super::parse_command_from_stripped("foo bar baz"), Some(
            super::GTPCommand{
                id: None,
                command: "foo".to_string(),
                args: "bar baz".to_string(),
            }));
        assert_eq!(super::parse_command_from_stripped("42 foo"), Some(
            super::GTPCommand{
                id: Some(42u32),
                command: "foo".to_string(),
                args: "".to_string(),
            }));
        assert_eq!(super::parse_command_from_stripped("42 foo bar baz"), Some(
            super::GTPCommand{
                id: Some(42u32),
                command: "foo".to_string(),
                args: "bar baz".to_string(),
            }));
    }

    #[test]
    fn parse_command() {
        assert_eq!(
            super::parse_command("  #  this is a comment\n\t  \n  # this as well"),
            None
        );
        assert_eq!(
            super::parse_command("#this command is really cool\n42 foo Cake is a lie  # cool isn't it ?"),
            Some(super::GTPCommand{
                id: Some(42u32),
                command: "foo".to_string(),
                args: "Cake is a lie ".to_string(),
            })
        );
    }

    #[test]
    fn arg_parse_colour() {
        assert_eq!(super::arg_parse_colour("BlAcK"), Some(api::Colour::Black));
        assert_eq!(super::arg_parse_colour("b"), Some(api::Colour::Black));
        assert_eq!(super::arg_parse_colour("WHIte"), Some(api::Colour::White));
        assert_eq!(super::arg_parse_colour("W"), Some(api::Colour::White));
        assert_eq!(super::arg_parse_colour("FOO"), None);
        assert_eq!(super::arg_parse_colour("bar"), None);
    }

    #[test]
    fn arg_parse_move() {
        assert_eq!(super::arg_parse_move("ReSiGn"), Some(api::Move::Resign));
        assert_eq!(super::arg_parse_move("PasS"), Some(api::Move::Pass));
        assert_eq!(super::arg_parse_move("A12"), Some(api::Move::Stone(api::Vertex::from_coords(1,12).unwrap())));
        assert_eq!(super::arg_parse_move("T7"), Some(api::Move::Stone(api::Vertex::from_coords(19,7).unwrap())));
        assert_eq!(super::arg_parse_move("F26"), None);
        assert_eq!(super::arg_parse_move("I13"), None);
        assert_eq!(super::arg_parse_move("foo"), None);

    }

    #[test]
    fn parse_args() {
        let arg_string = "W G7 alive black pass E5";
        let arg_types = vec!(
            super::ArgType::ColouredMoveArg,
            super::ArgType::StoneStatusArg,
            super::ArgType::ColourArg,
            super::ArgType::MoveArg,
            super::ArgType::VertexArg
        );
        let expected_args = vec!(
            super::Argument::ArgColouredMove(api::ColouredMove{player: api::Colour::White, mov: api::Move::Stone(api::Vertex::from_coords(7,7).unwrap())}),
            super::Argument::ArgStoneStatus(api::StoneStatus::Alive),
            super::Argument::ArgColour(api::Colour::Black),
            super::Argument::ArgMove(api::Move::Pass),
            super::Argument::ArgVertex(api::Vertex::from_coords(5,5).unwrap())
        );
        let parsed_args = super::parse_args(arg_string, &arg_types);
        assert_eq!(parsed_args.unwrap(), expected_args);
    }
}
