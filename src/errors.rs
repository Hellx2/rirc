/**
An error enum listing each error available in the crate.

## Items
- `ParsingError`: The error returned when parsing something fails.
*/
#[derive(Debug)]
pub enum IrcError {
    ParsingError,
}
