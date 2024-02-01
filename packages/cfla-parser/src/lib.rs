use crate::token::TokenStream;

pub mod error;
pub mod token;
pub mod module;
pub mod pair;
pub mod object;
pub mod list;
pub mod value;
pub mod stdlib;

pub trait Parse {
    fn parse<'a>(token_stream: &'a mut TokenStream<'a>) -> TokenStream<'a>;
}