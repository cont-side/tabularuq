use std::path::Path;

pub mod csv;
pub mod error;
pub mod xslx;

pub type TabularStringRecord = Vec<String>;

pub trait TabularCursor {
    fn cursor(
        &mut self,
    ) -> Result<impl Iterator<Item = TabularStringRecord>, Box<dyn std::error::Error>>;
}

pub trait TabularPorter {
    fn new<P>(src: P) -> Result<Self, Box<dyn std::error::Error>>
    where
        P: AsRef<Path>,
        Self: Sized;
}
