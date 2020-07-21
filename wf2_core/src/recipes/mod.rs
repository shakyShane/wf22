use crate::cli::CliCommand;
use crate::recipes::m2::M2;
use std::fmt::{Debug, Display};
use std::str::FromStr;
use anyhow::*;

pub mod m2;
pub mod wp;

pub trait Recipe: CliCommand + Debug + Display {}
//
// pub fn select(_name: Option<String>) -> Box<dyn Recipe> {
//     Box::new(M2)
// }

#[derive(Debug, Clone)]
pub enum RecipeKinds {
    M2, Wp
}

impl RecipeKinds {
    pub fn variants() -> [&'static str; 2] {
        ["M2", "Wp"]
    }
    pub fn select(&self) -> Box<dyn Recipe> {
        match self {
            RecipeKinds::M2 => Box::new(M2),
            _ => unimplemented!(),
        }
    }
}

impl FromStr for RecipeKinds {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "M2" | "m2" => Ok(RecipeKinds::M2),
            "Wp" | "WP" | "wp" => Ok(RecipeKinds::Wp),
            recipe_name => Err(anyhow!("unknown recipe: {}", recipe_name))
        }
    }
}
