use crate::error::ParseResult;
use crate::node::Element;

pub mod error;
pub mod node;
pub mod module;
pub mod pair;
pub mod object;
pub mod list;
pub mod value;
pub mod stdlib;

pub trait Parse {
    fn parse(tokens: &str) -> ParseResult<Element>;
}