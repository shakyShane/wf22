use crate::recipes::RecipeKinds;
use std::convert::TryFrom;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Context {
    cwd: std::path::PathBuf,
    recipe: Option<RecipeKinds>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            cwd: std::path::PathBuf::from("."),
            recipe: None,
        }
    }
}

impl Context {
    pub fn with_recipe(&mut self, recipe: Option<RecipeKinds>) {
        self.recipe = recipe;
    }
}
