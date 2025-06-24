#![allow(dead_code)]
// ANCHOR: example
/// Represents the possible states of the state machine.
#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    /// The initial state, indicating that the state machine is ready to start.
    Idle,
    /// The state indicating that the state machine is currently processing.
    Processing,
    /// The state indicating that the processing has been completed
    /// successfully.
    Completed,
    /// The state indicating that an error occurred during processing.
    Error,
}

/// Represents the state machine itself, holding the current state.
struct StateMachine {
    state: State,
}
/// Represents the possible events that can trigger state transitions.
#[derive(Debug)]
enum Event {
    Start,
    Finish,
    Fail,
    Reset,
}

/// Implementation of the StateMachine, including state transitions and state
/// retrieval.
impl StateMachine {
    /// Creates a new StateMachine in the Idle state.
    fn new() -> Self {
        StateMachine { state: State::Idle }
    }

    /// Transitions the state machine to a new state based on the current state
    /// and the received event.
    fn transition(&mut self, event: Event) {
        self.state = match (&self.state, event) {
            (State::Idle, Event::Start) => State::Processing,
            (State::Processing, Event::Finish) => State::Completed,
            (State::Processing, Event::Fail) => State::Error,
            (State::Processing, Event::Start) => State::Error, /* Illegal double start */
            (State::Completed, Event::Reset) => State::Idle,
            (State::Error, Event::Reset) => State::Idle,
            (_, _) => self.state, // Stay in the same state for other events
        };
    }

    /// Returns a reference to the current state of the state machine.
    fn get_state(&self) -> &State {
        &self.state
    }
}

fn main() {
    let mut state_machine = StateMachine::new();

    let st = state_machine.get_state();
    println!("Initial state: {:?}", st);
    assert_eq!(st, &State::Idle);

    state_machine.transition(Event::Start);
    let st = state_machine.get_state();
    println!("After Start: {:?}", st);
    assert_eq!(st, &State::Processing);

    state_machine.transition(Event::Finish);
    let st = state_machine.get_state();
    println!("After Finish: {:?}", st);
    assert_eq!(st, &State::Completed);

    state_machine.transition(Event::Reset);
    let st = state_machine.get_state();
    println!("After Reset: {:?}", st);
    assert_eq!(st, &State::Idle);

    state_machine.transition(Event::Start);
    let st = state_machine.get_state();
    println!("After Start: {:?}", st);
    assert_eq!(st, &State::Processing);

    state_machine.transition(Event::Fail);
    let st = state_machine.get_state();
    println!("After Fail: {:?}", st);
    assert_eq!(st, &State::Error);

    state_machine.transition(Event::Reset);
    let st = state_machine.get_state();
    println!("After Reset: {:?}", st);
    assert_eq!(st, &State::Idle);

    state_machine.transition(Event::Start);
    let st = state_machine.get_state();
    println!("After Start: {:?}", st);
    assert_eq!(st, &State::Processing);

    state_machine.transition(Event::Start);
    let st = state_machine.get_state();
    println!("After double start: {:?}", st);
    assert_eq!(st, &State::Error);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
