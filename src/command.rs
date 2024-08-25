/**
The set of commands for the IRC client.

## Items
- `Nick`: Change user nickname (?).
- `Join`: Join IRC channel(s).
*/
#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Nick(&'a str, Option<usize>),
    Join(Vec<Channel<'a>>),
    Names(Vec<&'a str>, Option<&'a str>),
}

/**
Implementation for an IRC channel.
*/
#[derive(Debug, PartialEq)]
pub struct Channel<'a> {
    pub name: &'a str,
    pub key: Option<&'a str>,
}
