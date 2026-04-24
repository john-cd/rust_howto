#!/bin/bash
# Modify crate_indices/src/main.rs to add uncategorized crates to "Uncategorized" section
sed -i '33a \                    if cats.is_empty() {\n                        cats.push(Category {\n                            category: "Uncategorized".into(),\n                            slug: "uncategorized".into(),\n                            description: "These crates are not added to any category on crates.io".into(),\n                        });\n                    }' tools/crate_indices/src/main.rs
