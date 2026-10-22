//! A plugin system that stores shared services by their `TypeId`.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;

/// A plugin that the host can run.
pub trait Plugin: Send + Sync + 'static {
    /// Returns the name of the plugin.
    fn name(&self) -> &'static str;

    /// Runs the plugin with access to the shared context.
    fn execute(&mut self, context: &mut Context);
}

/// The services shared between the host and the plugins, indexed by type.
#[derive(Default)]
pub struct Context {
    /// The services, by type.
    shared_data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Context {
    /// Returns the service of type `T`, if registered.
    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.shared_data
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
    }

    /// Returns a mutable reference to the service of type `T`, if registered.
    pub fn get_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
        self.shared_data
            .get_mut(&TypeId::of::<T>())
            .and_then(|b| b.downcast_mut())
    }

    /// Registers a service of type `T`, replacing any existing one.
    pub fn register<T: Any + Send + Sync>(&mut self, service: T) {
        self.shared_data
            .insert(TypeId::of::<T>(), Box::new(service));
    }
}

/// State owned by the host.
struct AppState {
    /// A counter that plugins can increase.
    counter: u32,
}

/// A plugin that increases the counter.
pub struct MyCounterPlugin;

impl Plugin for MyCounterPlugin {
    fn name(&self) -> &'static str {
        "MyCounterPlugin"
    }

    fn execute(&mut self, context: &mut Context) {
        if let Some(state) = context.get_mut::<AppState>() {
            state.counter += 10;
            println!("MyCounterPlugin: Increased counter to {}", state.counter);
        }
    }
}

/// A plugin that reads and registers strings.
pub struct MyStringPlugin;

impl Plugin for MyStringPlugin {
    fn name(&self) -> &'static str {
        "MyStringPlugin"
    }

    fn execute(&mut self, context: &mut Context) {
        if let Some(value) = context.get::<i32>() {
            println!("MyStringPlugin: Retrieved i32: {value}");
        }
        context.register("Plugin Says Hello!".to_string());
    }
}

/// A service registered by a plugin.
#[derive(Debug)]
pub struct CustomPluginService {
    /// A value.
    pub value: u64,
}

/// A plugin that registers and reads services.
pub struct MyServiceConsumerPlugin;

impl Plugin for MyServiceConsumerPlugin {
    fn name(&self) -> &'static str {
        "MyServiceConsumerPlugin"
    }

    fn execute(&mut self, context: &mut Context) {
        context.register(CustomPluginService { value: 12345 });
        if let Some(s) = context.get::<String>() {
            println!("MyServiceConsumerPlugin: Retrieved String: \"{s}\"");
        }
    }
}

/// Owns the plugins and their context.
#[derive(Default)]
struct PluginManager {
    /// The registered plugins.
    plugins: Vec<Box<dyn Plugin>>,
    /// The context shared by the plugins.
    context: Context,
}

impl PluginManager {
    /// Adds a plugin.
    fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    /// Runs every plugin once.
    fn initialize(&mut self) {
        for plugin in &mut self.plugins {
            println!("Initializing plugin '{}'", plugin.name());
            plugin.execute(&mut self.context);
        }
    }
}

/// Registers services, runs the plugins and checks the shared state.
fn main() {
    let mut manager = PluginManager::default();

    manager.context.register(AppState { counter: 0 });
    manager.context.register(100i32);
    manager.context.register("Host Greeting".to_string());

    manager.add_plugin(Box::new(MyCounterPlugin));
    manager.add_plugin(Box::new(MyStringPlugin));
    manager.add_plugin(Box::new(MyServiceConsumerPlugin));

    manager.initialize();

    println!("\n--- Host checking state after plugins ---");
    if let Some(state) = manager.context.get::<AppState>() {
        println!("Final app counter: {}", state.counter);
    }

    if let Some(s) = manager.context.get::<String>() {
        println!("Host retrieved String: \"{s}\"");
    }

    if let Some(svc) = manager.context.get::<CustomPluginService>() {
        println!("Host retrieved CustomPluginService: {}", svc.value);
    }
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
