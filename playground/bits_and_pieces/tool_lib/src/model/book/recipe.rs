use serde::Serialize;

use crate::Crate;

#[derive(Serialize, Debug)]
/// A recipe.
pub struct Recipe {
    name: String,
    crates: Vec<Crate>,
}

impl Recipe {
    /// Creates a new recipe.
    fn new(name: String, crates: Vec<Crate>) -> Self {
        Self { name, crates }
    }

    fn builder() -> RecipeBuilder {
        RecipeBuilder {
            inner: Recipe {
                name: String::new(),
                crates: Vec::new(),
            },
        }
    }
}

/// A builder for a recipe.
struct RecipeBuilder {
    inner: Recipe,
}

impl RecipeBuilder {
    /// Sets the name of the recipe.
    fn name(mut self, name: String) -> Self {
        self.inner.name = name;
        self
    }

    /// Adds a crate to the recipe.
    fn add_crate(mut self, crt: Crate) -> Self {
        self.inner.crates.push(crt);
        self
    }

    /// Build the `Recipe`.
    fn build(self) -> Recipe {
        self.inner
    }
}
