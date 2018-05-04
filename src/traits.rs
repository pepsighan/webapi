use result::GResult;
use std::io::Write;

pub trait Scrape: Sized {
    type From;

    fn scrape(from: &Self::From) -> Self;
}

pub trait WriteBindings {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()>;
}
