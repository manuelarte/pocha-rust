use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Suite {
    OROS,
    COPAS,
    BASTOS,
    ESPADAS
}

impl fmt::Display for Suite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Card {
    suite: Suite,
    number: u8,
}

impl Card {
    pub fn new(suite: Suite, number: u8) -> Self {
        Self { suite, number }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.suite, self.number)
    }
}