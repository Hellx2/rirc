use regex::Regex;

use command::{Channel, Command};
use errors::IrcError;
use message::Message;

/**
A function to parse input and parse as a command.

## TODO
- Allow for case-insensitive or lowercase commands.
*/
pub fn parse(msg_str: &str) -> Result<Message, IrcError> {
    // A regex to validate and parse commands.
    let re =
        Regex::new(r"^(:(?P<prefix>\S+) )?(?P<command>\S+)( (?P<params>.+?))?( :(?P<trail>.+))?$")
            .expect("Failed to create RegEx!");

    let (prefix, command, params, _) = match re.captures(msg_str) {
        Some(caps) => (
            caps.name("prefix"),
            caps.name("command"),
            caps.name("params"),
            caps.name("trail"),
        ),
        None => return Err(IrcError::ParsingError),
    };

    let params_splitted = split_params(params)?;

    let cmd = match command {
        Some("NICK") => parse_nick(params_splitted),
        Some("JOIN") => parse_join(params_splitted),
        Some("NAMES") => parse_names(params_splitted),
        _ => return Err(IrcError::ParsingError),
    };

    match cmd {
        Ok(c) => Ok(Message {
            source: prefix,
            cmd: c,
        }),
        Err(e) => Err(e),
    }
}

/// Function to split the parameters of a command for parsing.
fn split_params(params: Option<&str>) -> Result<Vec<&str>, IrcError> {
    match params {
        Some(p) => Ok(p.split(' ').collect()),
        None => Err(IrcError::ParsingError),
    }
}

/// Function for parsing the `names` command.
fn parse_names(params: Vec<&str>) -> Result<Command, IrcError> {
    let channel_names: Vec<&str> = match params.first() {
        Some(v) => v.split(',').collect(),
        None => return Err(IrcError::ParsingError),
    };

    Ok(Command::Names(channel_names, params.get(1).copied()))
}

/// Function for parsing the `join` command.
fn parse_join(params: Vec<&str>) -> Result<Command, IrcError> {
    let channel_names: Vec<&str> = match params.first() {
        Some(v) => v.split(',').collect(),
        None => return Err(IrcError::ParsingError),
    };

    let keys: Vec<&str> = match params.get(1) {
        Some(v) => v.split(',').collect(),
        None => Vec::new(),
    };

    Ok(Command::Join(
        channel_names
            .iter()
            .enumerate()
            .map(|(i, v)| Channel {
                name: v,
                key: keys.get(i).copied(),
            })
            .collect(),
    ))
}

/// Function for parsing the `nick` command.
fn parse_nick(params: Vec<&str>) -> Result<Command, IrcError> {
    let nick = match params.first() {
        Some(nick) => *nick,
        None => return Err(IrcError::ParsingError),
    };

    let hopcount = match params.get(1) {
        Some(v) => match ::std::str::FromStr::from_str(v) {
            Ok(h) => Some(h),
            _ => None, // second param not possible to convert to usize, but keep on rolling.
        },
        None => None,
    };

    Ok(Command::Nick(nick, hopcount))
}
