use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Could not write data because: {:?}", _0)]
    FailedToWrite(#[cause] io::Error)
}

pub type GResult<T> = Result<T, Error>;
