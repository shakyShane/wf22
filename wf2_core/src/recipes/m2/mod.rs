use crate::recipes::Recipe;
use std::fmt;
use std::fmt::Formatter;

pub mod m2_cli;
pub mod subcommands;

#[derive(Debug)]
pub struct M2;

impl fmt::Display for M2 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", "Magento 2")
    }
}

impl Recipe for M2 {}

