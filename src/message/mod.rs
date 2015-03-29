use command::Command;
use errors::IrcError;

mod parser;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Message {
    pub source: Option<String>,
    pub cmd: Command,
}

impl Message {
    pub fn new(msg: &str) -> Result<Message,IrcError> {
        return parser::parse(msg);
    }
}

