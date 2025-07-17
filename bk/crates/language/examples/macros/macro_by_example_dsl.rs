// ANCHOR: example
//! This macro provides a domain-specific language (DSL) for defining
//! configuration for an application. Instead of using a HashMap, or a struct
//! directly, we want a more declarative, human-readable way to specify
//! settings.

use std::collections::HashMap;

// Represent the types of values our configuration can hold.
#[derive(Debug, PartialEq, Clone)]
pub enum ConfigValue {
    String(String),
    Number(i64),
    Boolean(bool),
    Map(HashMap<String, ConfigValue>),
}

// Our DSL Macro.
//
// The `#[macro_export]` attribute makes the `config!` macro available to other
// crates.
#[macro_export]
macro_rules! config {
    // Base case: no more tokens, return an empty HashMap.
    () => {
        std::collections::HashMap::new()
    };

    // Rule for a string literal value (e.g., `key: "value"`).
    // - Note the use of recursive macro calls.
    //   Each rule builds a `HashMap` by inserting the current key-value pair
    //   and then merging it with the `HashMap` returned by the recursive call to `config!`.
    // - `stringify!` is used to convert the identifier into a string slice:
    //   `stringify!(1 + 1) == "1 + 1"`
    // - The keyword metavariable `$crate` refers to the crate defining the macro;
    //   It is used to refer to items or macros that are not in scope at the invocation site, here the enum defined above.
    //   See <https://doc.rust-lang.org/reference/macros-by-example.html#r-macro.decl.hygiene.crate>.
    ($key:ident : $value:literal, $($rest:tt)*) => {
        {
            let mut map = config!($($rest)*);
            map.insert(stringify!($key).to_string(), $crate::macro_by_example_dsl::ConfigValue::String($value.to_string()));
            map
        }
    };
    ($key:ident : $value:literal) => {
        {
            let mut map = std::collections::HashMap::new();
            map.insert(stringify!($key).to_string(), $crate::macro_by_example_dsl::ConfigValue::String($value.to_string()));
            map
        }
    };

    // Rule for a boolean value (e.g., `key: true`).
    // `:` and `true` or `false` are literals in the macro-by-example pattern.
    ($key:ident : true, $($rest:tt)*) => {
        {
            let mut map = config!($($rest)*);
            map.insert(stringify!($key).to_string(), $crate::macro_by_example_dsl::ConfigValue::Boolean(true));
            map
        }
    };
    ($key:ident : false, $($rest:tt)*) => {
        {
            let mut map = config!($($rest)*);
            map.insert(stringify!($key).to_string(), $crate::macro_by_example_dsl::ConfigValue::Boolean(false));
            map
        }
    };
    ($key:ident : true) => {
        {
            let mut map = std::collections::HashMap::new();
            map.insert(stringify!($key).to_string(), $crate::macro_by_example_dsl::ConfigValue::Boolean(true));
            map
        }
    };
    ($key:ident : false) => {
        {
            let mut map = std::collections::HashMap::new();
            map.insert(stringify!($key).to_string(), $crate::macro_by_example_dsl::ConfigValue::Boolean(false));
            map
        }
    };

    // Rule for a number value (e.g., `key: 123`).
    ($key:ident : $value:expr, $($rest:tt)*) => {
        {
            // `$value` is parsed as an expression. We rely on it being an integer literal.
            let mut map = config!($($rest)*);
            map.insert(stringify!($key).to_string(), $crate::macro_by_example_dsl::ConfigValue::Number($value as i64));
            map
        }
    };
    ($key:ident : $value:expr) => {
        {
            let mut map = std::collections::HashMap::new();
            map.insert(stringify!($key).to_string(), $crate::macro_by_example_dsl::ConfigValue::Number($value as i64));
            map
        }
    };

    // Rule for a nested section (identifier followed by block), followed by a comma.
    ($key:ident { $($inner_config:tt)* }, $($rest:tt)*) => {
        {
            let mut map = config!($($rest)*);
            map.insert(stringify!($key).to_string(), $crate::macro_by_example_dsl::ConfigValue::Map(config!($($inner_config)*)));
            map
        }
    };

    // Rule for a nested section without a trailing comma (last item).
    ($key:ident { $($inner_config:tt)* }) => {
        {
            let mut map = std::collections::HashMap::new();
            map.insert(stringify!($key).to_string(), $crate::macro_by_example_dsl::ConfigValue::Map(config!($($inner_config)*)));
            map
        }
    };
}

fn main() {
    // Example usage of the `config!` macro to set up application settings in a
    // declarative way.
    //
    // The macro allows for nested structures, making it easy to define complex
    // configurations. The result will be a `HashMap` with keys as strings
    // and values as `ConfigValue` enums, which can represent strings,
    // numbers, booleans, or nested maps.
    let app_config: HashMap<String, ConfigValue> = config! {
        debug_mode: true,
        log_level: "info",
        max_connections: 50,

        database {
            host: "localhost",
            port: 5432,
            username: "admin",
            password: "secure_password",
            pool_size: 10
        },

        network {
            protocol: "tcp",
            timeout_seconds: 30,
            retries: 3
        }
    };

    println!("{:#?}", app_config);

    // You can now access values like this:
    if let Some(ConfigValue::Boolean(debug)) = app_config.get("debug_mode") {
        println!("Debug mode: {}", debug);
    }

    if let Some(ConfigValue::Map(db_config)) = app_config.get("database") {
        if let Some(ConfigValue::Number(port)) = db_config.get("port") {
            println!("Database port: {}", port);
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
