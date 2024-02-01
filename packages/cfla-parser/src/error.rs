pub type ParserResult<T> = Result<T, Error>;

pub enum Error {
    Unexpected { given: Option<String> },
}