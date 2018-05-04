pub trait Scrape: Sized {
    type From;

    fn scrape(from: &Self::From) -> Self;
}
