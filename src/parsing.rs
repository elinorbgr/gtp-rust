use std::u32;
use std::vec::Vec;
use std::ascii::Ascii;

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
    id: Option<u32>,
    command: Vec<Ascii>,
    args: Vec<Ascii>
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
        let id = u32::parse_bytes(first_part.as_str_ascii().as_bytes(), 10);
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
#[allow(dead_code)]
pub fn parse_command(input: &[Ascii]) -> Option<GTPCommand> {
    match strip_input(input).as_slice().splitn(1, |&c| {c == '\n'.to_ascii()}).next() {
        Some(line) => parse_command_from_stripped(line),
        _ => None
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn strip_input() {
        let input = "command1 \t and \x15 argu\x07ments\n# this is a comment\n  command2    !op # comment\n\n\nfoo bar\n\n".to_ascii();
        let expected_output = "command1 and arguments\ncommand2 !op \nfoo bar\n".to_ascii();
        let output = super::strip_input(input);
        assert_eq!(output.as_slice(), expected_output);
    }

    #[test]
    fn parse_command_from_stripped() {
        assert_eq!(super::parse_command_from_stripped(" ".to_ascii()), None);
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
}
