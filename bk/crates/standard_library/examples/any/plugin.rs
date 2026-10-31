// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A plugin system that stores shared services by their `TypeId`.
//!
//! A service registry stores one object per type, keyed by `TypeId`, and
//! plugins look up the services they need. It is the idea behind
//! dependency-injection containers. Downcasting back to the real type checks
//! the type at run time.

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
        context.register("Plugin Says Hello!".to_owned());
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
    manager.context.register("Host Greeting".to_owned());

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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_example_entry_point() {
        main();
    }

    #[test]
    fn test_context_register_get_and_get_mut()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut ctx = Context::default();
        assert!(ctx.get::<i32>().is_none());
        ctx.register(7_i32);
        assert_eq!(ctx.get::<i32>(), Some(&7));
        *ctx.get_mut::<i32>().ok_or("no i32 registered")? += 1;
        assert_eq!(ctx.get::<i32>(), Some(&8));
        // Services are keyed by type.
        assert!(ctx.get::<u32>().is_none());
        // Registering again replaces the previous value.
        ctx.register(1_i32);
        assert_eq!(ctx.get::<i32>(), Some(&1));
        Ok(())
    }

    #[test]
    fn test_plugins_cooperate_through_the_context()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut manager = PluginManager::default();
        manager.context.register(AppState { counter: 0 });
        manager.context.register(100i32);
        manager.context.register("Host Greeting".to_owned());
        manager.add_plugin(Box::new(MyCounterPlugin));
        manager.add_plugin(Box::new(MyStringPlugin));
        manager.add_plugin(Box::new(MyServiceConsumerPlugin));
        manager.initialize();

        let state = manager.context.get::<AppState>().ok_or("no AppState")?;
        assert_eq!(state.counter, 10);
        // `MyStringPlugin` replaced the host's greeting.
        assert_eq!(
            manager.context.get::<String>().map(String::as_str),
            Some("Plugin Says Hello!")
        );
        let service = manager
            .context
            .get::<CustomPluginService>()
            .ok_or("no CustomPluginService")?;
        assert_eq!(service.value, 12345);
        Ok(())
    }

    #[test]
    fn test_plugin_names() {
        assert_eq!(MyCounterPlugin.name(), "MyCounterPlugin");
        assert_eq!(MyStringPlugin.name(), "MyStringPlugin");
        assert_eq!(MyServiceConsumerPlugin.name(), "MyServiceConsumerPlugin");
    }

    #[test]
    fn test_plugin_without_its_state_does_nothing() {
        let mut ctx = Context::default();
        MyCounterPlugin.execute(&mut ctx);
        assert!(ctx.get::<AppState>().is_none());
    }
}
