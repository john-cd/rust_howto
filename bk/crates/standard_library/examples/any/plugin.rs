use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;

pub trait Plugin: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn execute(&mut self, context: &mut Context);
}

#[derive(Default)]
pub struct Context {
    shared_data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Context {
    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.shared_data
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref())
    }

    pub fn get_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
        self.shared_data
            .get_mut(&TypeId::of::<T>())
            .and_then(|b| b.downcast_mut())
    }

    pub fn register<T: Any + Send + Sync>(&mut self, service: T) {
        self.shared_data
            .insert(TypeId::of::<T>(), Box::new(service));
    }
}

struct AppState {
    counter: u32,
}

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

#[derive(Debug)]
pub struct CustomPluginService {
    pub value: u64,
}

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

#[derive(Default)]
struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    context: Context,
}

impl PluginManager {
    fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    fn initialize(&mut self) {
        for plugin in &mut self.plugins {
            println!("Initializing plugin '{}'", plugin.name());
            plugin.execute(&mut self.context);
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
