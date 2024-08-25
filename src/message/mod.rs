use command::Command;
use errors::IrcError;

mod parser;

#[cfg(test)]
mod tests;

/**
Struct symbolic of a sent message.

## Notes
- `cmd` should likely be an `Option`
*/
#[derive(Debug)]
pub struct Message<'a> {
    pub source: Option<&'a str>,
    pub cmd: Command<'a>,
}

impl<'a> Message<'a> {
    pub fn new(msg: &str) -> Result<Message, IrcError> {
        parser::parse(msg)
    }
}
