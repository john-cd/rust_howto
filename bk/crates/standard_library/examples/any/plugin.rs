// // [clean up / shorten](https://github.com/john-cd/rust_howto/issues/1412)

// mod plugin_interface {

//     use std::any::Any;

//     /// Trait for a general plugin.
//     /// A plugin might have a name and a method to execute its primary logic.
//     pub trait Plugin: Send + Sync + 'static {
//         // `'static` is required for `Any`
//         fn name(&self) -> &'static str;
//         fn execute(&self, context: &mut dyn PluginContext);
//     }

//     /// Trait that the host implements, allowing plugins to interact with the
//     /// host.
//     /// Plugins can request host services or data through this context.
//     pub trait PluginContext: Any + Send + Sync + 'static {
//         // Methods for plugins to get data from the host.
//         fn get_data<T: Any + Send + Sync>(&self) -> Option<&T>;
//         fn get_data_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T>;

//         // Methods for plugins to register their own data (e.g., services).
//         fn register_service<T: Any + Send + Sync>(&mut self, service: T);

//         // Example of a host-provided logging service.
//         fn log(&mut self, message: &str);
//     }

//     // Helper trait to allow downcasting `&dyn Plugin` to `&dyn Any`
//     // because `Plugin` itself doesn't directly implement `Any`.
//     pub trait AsAny {
//         fn as_any(&self) -> &dyn Any;
//         fn as_any_mut(&mut self) -> &mut dyn Any;
//     }

//     impl<T: Any + Plugin> AsAny for T {
//         fn as_any(&self) -> &dyn Any {
//             self
//         }

//         fn as_any_mut(&mut self) -> &mut dyn Any {
//             self
//         }
//     }
// }

// mod counter {

//     use std::any::Any;
//     use std::sync::Arc;
//     use std::sync::Mutex;

//     use plugin_interface::Plugin;
//     use plugin_interface::PluginContext; // For AppState

//     // --- Re-declare AppState from host (struct definitions must match) ---
//     // In a real scenario, if `AppState` were part of the shared
//     // `plugin_interface`, we wouldn't need to re-declare it. If it's truly
//     // host-internal, then plugins access it via a service that wraps it.
//     // For this example, we assume `AppState` is a concrete type known to the
//     // plugin.
//     struct AppState {
//         counter: u32,
//         messages: Vec<String>,
//     }

//     pub struct MyCounterPlugin;

//     impl Plugin for MyCounterPlugin {
//         fn name(&self) -> &'static str {
//             "MyCounterPlugin"
//         }

//         fn execute(&self, context: &mut dyn PluginContext) {
//             context.log("MyCounterPlugin: Executing...");

//             // Try to get mutable access to the host's AppState
//             if let Some(app_state_mutex) =
//                 context.get_data_mut::<Arc<Mutex<AppState>>>()
//             {
//                 let mut app_state = app_state_mutex.lock().unwrap();
//                 app_state.counter += 10;
//                 context.log(&format!(
//                     "MyCounterPlugin: Increased counter to {}",
//                     app_state.counter
//                 ));
//             } else {
//                 context.log("MyCounterPlugin: Could not access AppState!");
//             }
//         }
//     }
// }

// mod string {

//     use std::any::Any;

//     use plugin_interface::Plugin;
//     use plugin_interface::PluginContext;

//     pub struct MyStringPlugin;

//     impl Plugin for MyStringPlugin {
//         fn name(&self) -> &'static str {
//             "MyStringPlugin"
//         }

//         fn execute(&self, context: &mut dyn PluginContext) {
//             context.log("MyStringPlugin: Executing...");

//             // Try to get an i32 from the host
//             if let Some(value) = context.get_data::<i32>() {
//                 context.log(&format!(
//                     "MyStringPlugin: Retrieved i32 from host: {}",
//                     value
//                 ));
//             } else {
//                 context
//                     .log("MyStringPlugin: Could not retrieve i32 from
// host.");             }

//             // Try to get a String from the host
//             if let Some(s) = context.get_data::<String>() {
//                 context.log(&format!(
//                     "MyStringPlugin: Retrieved String from host: \"{}\"",
//                     s
//                 ));
//             } else {
//                 context.log(
//                     "MyStringPlugin: Could not retrieve String from host.",
//                 );
//             }

//             // Register a new String service provided by this plugin
//             let plugin_message = "Plugin Says Hello!".to_string();
//             context.register_service(plugin_message);
//         }
//     }
// }

// mod consumer {

//     use std::any::Any;

//     use plugin_interface::Plugin;
//     use plugin_interface::PluginContext;

//     // A custom service type defined by this plugin
//     #[derive(Debug)]
//     pub struct CustomPluginService {
//         pub value: u64,
//     }

//     pub struct MyServiceConsumerPlugin;

//     impl Plugin for MyServiceConsumerPlugin {
//         fn name(&self) -> &'static str {
//             "MyServiceConsumerPlugin"
//         }

//         fn execute(&self, context: &mut dyn PluginContext) {
//             context.log("MyServiceConsumerPlugin: Executing...");

//             // This plugin registers its own service first:
//             context.register_service(CustomPluginService { value: 12345 });

//             // Then, it tries to get a String service, potentially from
// another             // plugin.
//             if let Some(s) = context.get_data::<String>() {
//                 context.log(&format!(
//                     "MyServiceConsumerPlugin: Retrieved String from context:
// \"{}\"",                     s
//                 ));
//             } else {
//                 context
//                     .log("MyServiceConsumerPlugin: String service not
// found.");             }
//         }
//     }
// }

// // In real life, the following should be in a separate file / crate:
// use std::any::Any;
// use std::any::TypeId;
// use std::collections::HashMap;
// use std::sync::Arc;
// use std::sync::Mutex;

// use plugin_interface::AsAny;
// use plugin_interface::Plugin;
// use plugin_interface::PluginContext;

// // --- Host's internal data ---
// struct AppState {
//     counter: u32,
//     messages: Vec<String>,
// }

// // --- Host's implementation of `PluginContext` ---
// struct HostPluginContext {
//     // Allow mutable access to host-owned data that plugins might want to
//     // access/modify. The key is `TypeId`, and the value is a `Box<dyn Any +
//     // Send + Sync>`.
//     shared_data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
//     // We can also have direct fields for host-specific services
//     // that don't need to be dynamically discovered with `Any`.
//     app_state: Arc<Mutex<AppState>>,
// }

// impl HostPluginContext {
//     fn new(app_state: Arc<Mutex<AppState>>) -> Self {
//         HostPluginContext {
//             shared_data: HashMap::new(),
//             app_state,
//         }
//     }
// }

// impl PluginContext for HostPluginContext {
//     fn get_data<T: Any + Send + Sync>(&self) -> Option<&T> {
//         let type_id = TypeId::of::<T>();
//         self.shared_data
//             .get(&type_id)
//             .and_then(|boxed_any| boxed_any.downcast_ref::<T>())
//     }

//     fn get_data_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
//         let type_id = TypeId::of::<T>();
//         self.shared_data
//             .get_mut(&type_id)
//             .and_then(|boxed_any| boxed_any.downcast_mut::<T>())
//     }

//     fn register_service<T: Any + Send + Sync>(&mut self, service: T) {
//         let type_id = TypeId::of::<T>();
//         self.shared_data.insert(type_id, Box::new(service));
//         println!(
//             "Host: Registered service of type {:?}",
//             std::any::type_name::<T>()
//         );
//     }

//     fn log(&mut self, message: &str) {
//         println!("Host Log: {}", message);
//         let mut app_state = self.app_state.lock().unwrap();
//         app_state.messages.push(format!("[LOG] {}", message));
//     }
// }

// // --- Plugin Management ---
// struct PluginManager {
//     plugins: Vec<Box<dyn Plugin>>,
//     plugin_context: HostPluginContext,
// }

// impl PluginManager {
//     fn new(app_state: Arc<Mutex<AppState>>) -> Self {
//         PluginManager {
//             plugins: Vec::new(),
//             plugin_context: HostPluginContext::new(app_state),
//         }
//     }

//     fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
//         println!("Host: Loading plugin: {}", plugin.name());
//         self.plugins.push(plugin);
//     }

//     fn initialize_plugins(&mut self) {
//         for plugin in self.plugins.iter_mut() {
//             println!("Host: Initializing plugin '{}'", plugin.name());
//             plugin.execute(&mut self.plugin_context);
//         }
//     }

//     // Example of a host service that can be accessed by plugins.
//     fn register_host_service<T: Any + Send + Sync + 'static>(
//         &mut self,
//         service: T,
//     ) {
//         self.plugin_context.register_service(service);
//     }

//     // Host can also access services registered by plugins, if it knows their
//     // type.
//     fn get_registered_service<T: Any + Send + Sync + 'static>(
//         &self,
//     ) -> Option<&T> {
//         self.plugin_context.get_data::<T>()
//     }
// }

// fn main() {
//     let app_state = Arc::new(Mutex::new(AppState {
//         counter: 0,
//         messages: Vec::new(),
//     }));

//     let mut plugin_manager = PluginManager::new(Arc::clone(&app_state));

//     // --- Register host services ---
//     let initial_value = 100;
//     plugin_manager.register_host_service(initial_value); // Register an i32.
//     plugin_manager.register_host_service("Host Greeting".to_string());
//     // Register a String.

//     // --- Load plugins ---
//     // For this example, we'll manually create instances of our "simulated"
//     // plugins. In a real system, you'd use `libloading` to open a `.so` or
//     // `.dll` and then call an `extern "C"` function to get a `Box<dyn
//     // Plugin>`.

//     // Plugin 1: Increases a counter and logs.
//     plugin_manager.add_plugin(Box::new(MyCounterPlugin {}));

//     // Plugin 2: Reads a host-provided string and registers its own string.
//     plugin_manager.add_plugin(Box::new(MyStringPlugin {}));

//     // Plugin 3: Tries to get a custom service from another plugin.
//     plugin_manager.add_plugin(Box::new(MyServiceConsumerPlugin {}));

//     // --- Execute plugins ---
//     plugin_manager.initialize_plugins();

//     // --- Host checks the state after plugins run ---
//     println!("\n--- Host checking state after plugins ---");
//     let current_counter = app_state.lock().unwrap().counter;
//     println!("Final app counter: {}", current_counter);

//     let messages = app_state.lock().unwrap().messages.clone();
//     println!("App messages: {:?}", messages);

//     // Host tries to get a service registered by `MyStringPlugin`.
//     if let Some(plugin_message) =
//         plugin_manager.get_registered_service::<String>()
//     {
//         // This will actually get "Host Greeting" not "Plugin Says Hello"
//         // because we registered "Host Greeting" *after* initializing
//         // `HostPluginContext` and before any plugins ran.
//         // To get "Plugin Says Hello", the host would need to know to look
// for         // it *after* MyStringPlugin has registered it.
//         println!("Host retrieved String from context: \"{}\"",
// plugin_message);     }

//     if let Some(other_plugin_message) =
//         plugin_manager.get_registered_service::<CustomPluginService>()
//     {
//         println!(
//             "Host retrieved CustomPluginService from context: {}",
//             other_plugin_message.value
//         );
//     }
// }
