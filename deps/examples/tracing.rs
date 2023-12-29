use tracing::{event, error, warn, info, debug, trace, Level};

fn main() {
    event!(Level::INFO, "something happened");
    error!("!");
    warn!("!");
    info!("!");
    debug!("!");
    trace!("!");

    event!(target: "app_events", Level::TRACE, "something has happened!");

    // records an event with two fields (also works for spans)
    event!(Level::INFO, answer = 42, question = "life, the universe, and everything");

    // unlike other fields, `message`'s shorthand initialization is just the string itself.
    debug!(excitement = "yay!", "hello!");

    // shorthand for user = user
    let user = "ferris";
    event!(Level::TRACE, "login: {}", user);

    // `my_struct` will be recorded using its `fmt::Debug` implementation.
    let my_struct = S;
    event!(Level::TRACE, greeting = ?my_struct);
}

#[derive(Debug)]
struct S;
