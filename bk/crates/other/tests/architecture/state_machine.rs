// ANCHOR: example
#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Idle,
    Processing,
    Completed,
    Error,
}

struct StateMachine {
    state: State,
}

#[derive(Debug)]
enum Event {
    Start,
    Finish,
    Fail,
    Reset,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine { state: State::Idle }
    }

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
