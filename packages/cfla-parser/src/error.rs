pub type ParseResult<T> = Result<T, Error>;

pub enum Error {
    Unexpected { given: Option<String> },
}