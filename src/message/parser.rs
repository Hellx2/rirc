use regex::Regex;

use command::{Command,Channel};
use message::Message;
use errors::IrcError;

static SPACE: &'static str = " ";

pub fn parse(msg_str: &str) -> Result<Message,IrcError> {
    let re = Regex::new(r"^(:(?P<prefix>\S+) )?(?P<command>\S+)( (?P<params>.+?))?( :(?P<trail>.+))?$").unwrap();

    let (prefix, command, params, _) = match re.captures(msg_str) {
        Some(caps) => {
            (caps.name("prefix"), caps.name("command"), caps.name("params"), caps.name("trail"))
        },
        None => return Err(IrcError::ParsingError)
    };

    let params_splitted = try!(split_params(params));

    let cmd = match command {
        Some("NICK") => parse_nick(params_splitted),
        Some("JOIN") => parse_join(params_splitted),
        Some("NAMES") => parse_names(params_splitted),
        _ => return Err(IrcError::ParsingError)
    };

    match cmd {
        Ok(c) => Ok(Message {source: prefix.map(|p| p.to_string()), cmd: c}),
        Err(e) => Err(e)
    }
}

fn split_params(params: Option<&str>) -> Result<Vec<&str>,IrcError> {
    match params {
        Some(p) => Ok(p.split(SPACE).collect()),
        None => Err(IrcError::ParsingError)
    }
}

fn parse_names(params: Vec<&str>) -> Result<Command,IrcError> {
    let channel_names: Vec<String> = match params.get(0) {
        Some(v) => v.split(",").map(|s| s.to_string()).collect(),
        None => return Err(IrcError::ParsingError)
    };

    Ok(Command::NAMES(channel_names, params.get(1).map(|s| s.to_string())))
}

fn parse_join(params: Vec<&str>) -> Result<Command,IrcError> {
    let channel_names: Vec<&str> = match params.get(0) {
        Some(v) => v.split(",").collect(),
        None => return Err(IrcError::ParsingError)
    };

    let keys: Vec<&str> = match params.get(1) {
        Some(v) => v.split(",").collect(),
        None => Vec::new()
    };

    Ok(Command::JOIN(
            channel_names
            .iter()
            .enumerate()
            .map(|(i,v)| Channel { name: v.to_string(), key: keys.get(i).map(|s| s.to_string()) })
            .collect()))
}

fn parse_nick(params: Vec<&str>) -> Result<Command,IrcError> {
    let nick = match params.get(0) {
        Some(nick) => nick.to_string(),
        None => return Err(IrcError::ParsingError)
    };

    let hopcount = match params.get(1) {
        Some(v) => match ::std::str::FromStr::from_str(v) {
            Ok(h) => Some(h),
            _ => None // second param not possible to convert to usize, but keep on rolling.
        },
        None => None
    };

    Ok(Command::NICK(nick, hopcount))
}

