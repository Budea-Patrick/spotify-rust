use core::fmt;
use std::fmt::Formatter;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Image {
    height: u16,
    pub(crate) url: String,
    width: u16,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} - {}", self.height, self.url, self.width)
    }
}
