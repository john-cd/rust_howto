use serde::Serialize;

use crate::Crate;

#[derive(Serialize, Debug)]
pub struct Recipe {
    name: String,
    crates: Vec<Crate>,
}

impl Recipe {
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

struct RecipeBuilder {
    inner: Recipe,
}

impl RecipeBuilder {
    fn name(mut self, name: String) -> Self {
        self.inner.name = name;
        self
    }

    fn add_crate(mut self, crt: Crate) -> Self {
        self.inner.crates.push(crt);
        self
    }

    fn build(self) -> Recipe {
        self.inner
    }
}
