#[derive(Debug, PartialEq)]
pub enum Command {
    NICK(String, Option<usize>),
    JOIN(Vec<Channel>),
    NAMES(Vec<String>, Option<String>),
}

#[derive(Debug, PartialEq)]
pub struct Channel {
    pub name: String,
    pub key: Option<String>,
}

