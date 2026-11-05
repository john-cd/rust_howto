// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Event sourcing: store events, rebuild the state by replaying them.
//!
//! Event sourcing stores every change as an immutable event instead of
//! overwriting the current state. The state is rebuilt by replaying the events.
//! This gives a complete audit trail and the ability to rebuild past states, as
//! in banking ledgers.

/// Facts that happened to an account. Events are never modified.
#[derive(Debug, Clone)]
enum AccountEvent {
    Deposited(i64),
    Withdrawn(i64),
}

/// The current state, derived from the events.
#[derive(Debug, Default, PartialEq)]
struct Account {
    balance: i64,
}

impl Account {
    fn apply(&mut self, event: &AccountEvent) {
        match event {
            AccountEvent::Deposited(amount) => self.balance += amount,
            AccountEvent::Withdrawn(amount) => self.balance -= amount,
        }
    }

    /// Replays the whole history (or a prefix of it).
    fn replay(events: &[AccountEvent]) -> Self {
        let mut account = Self::default();
        for event in events {
            account.apply(event);
        }
        account
    }
}

/// Appends events to a log and rebuilds the state at different points in time.
fn main() {
    let log = vec![
        AccountEvent::Deposited(100),
        AccountEvent::Withdrawn(30),
        AccountEvent::Deposited(5),
    ];

    assert_eq!(Account::replay(&log).balance, 75);
    // Time travel: the state after the first two events.
    assert_eq!(Account::replay(&log[..2]).balance, 70);
    println!("{:?}", Account::replay(&log));
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the event sourcing example through its entry point.
    #[test]
    fn test_event_sourcing_example_entry_point() {
        main();
    }
}
