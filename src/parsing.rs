use std::u32;
use std::string::String;

// Strips all ignored content from input string
// according to specifications of GTPv2
fn strip_input(input: &str) -> String {
    let mut output = String::new();
    let mut last_char = 10 as char; // set initial lastchar as LF
    let mut in_comment = false;
    for c in input.chars() {
        if (c as u8) < 9 ||
           ((c as u8) > 10 && (c as u8) < 32) ||
           (c as u8) == 127 {
            // all non printable chars are discarded
            continue;
        }
        if (c as u8) == 10 { // newline
            in_comment = false;
            if (last_char as u8) == 10 {
                // ignore multiple newlines
                continue;
            }
            last_char = 10 as char;
            output = output.append((10 as char).to_string().as_slice());
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
        if c == ' ' || (c as u8) == 9 { // space or \t
            if last_char.is_whitespace() {
                // multiple spaces are discarded
                // as well as spaces in the begining of a line
                continue;
            }
            last_char = ' ';
            output = output.append(' '.to_string().as_slice());
            continue;
        }
        // any other character is kept as it
        last_char = c;
        output = output.append(c.to_string().as_slice());
    }
    output
}

#[deriving(PartialEq, Show)]
pub struct GTPCommand {
    id: Option<u32>,
    command: String,
    args: String
}

// parses a command from a line
// input is suposed to be a single stripped line
// without trailing \n
fn parse_command_from_stripped(line: &str) -> Option<GTPCommand> {
    if line.is_whitespace() {
        // empty line no command to parse
        None
    } else {
        let mut first_split = line.splitn(1, ' ');
        // there is always a first value
        let first_part = first_split.next().unwrap();
        let mut to_split = match first_split.next() {
            Some(text) => text,
            _ => ""
        };
        let id = u32::parse_bytes(first_part.as_bytes(), 10);
        match id {
            None => { to_split = line; },
            _ => { }
        };
        let mut second_split = to_split.splitn(1, ' ');
        match second_split.next() {
            Some(text) if text == "" => None,
            Some(text) => Some(GTPCommand{
                id: id,
                command: String::from_str(text),
                args: match second_split.next() {
                    Some(arguments) => String::from_str(arguments),
                    _ => String::from_str("")
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
pub fn parse_command(input: &str) -> Option<GTPCommand> {
    match strip_input(input).as_slice().splitn(1, 10 as char).next() {
        Some(line) => parse_command_from_stripped(line),
        _ => None
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn strip_input() {
        let input = "command1 \t and \x15 argu\x07ments\n# this is a comment\ncommand2    !op # comment\n\n\nfoo bar\n\n";
        let expected_output = "command1 and arguments\ncommand2 !op \nfoo bar\n";
        let output = super::strip_input(input);
        assert_eq!(output.as_slice(), expected_output);
    }

    #[test]
    fn parse_command_from_stripped() {
        assert_eq!(super::parse_command_from_stripped(" "), None);
        assert_eq!(super::parse_command_from_stripped("56"), None);
        assert_eq!(super::parse_command_from_stripped("foo"), Some(
            super::GTPCommand{
                id: None,
                command: String::from_str("foo"),
                args: String::from_str("")
            }));
        assert_eq!(super::parse_command_from_stripped("foo bar baz"), Some(
            super::GTPCommand{
                id: None,
                command: String::from_str("foo"),
                args: String::from_str("bar baz")
            }));
        assert_eq!(super::parse_command_from_stripped("42 foo"), Some(
            super::GTPCommand{
                id: Some(42u32),
                command: String::from_str("foo"),
                args: String::from_str("")
            }));
        assert_eq!(super::parse_command_from_stripped("42 foo bar baz"), Some(
            super::GTPCommand{
                id: Some(42u32),
                command: String::from_str("foo"),
                args: String::from_str("bar baz")
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
                command: String::from_str("foo"),
                args: String::from_str("Cake is a lie ")
            })
        );
    }
}
