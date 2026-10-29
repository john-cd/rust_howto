// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Simulate a single-server bank queue with a timestamp-ordered event heap.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

/// A customer's scheduled arrival and fixed service duration.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Customer {
    id: usize,
    arrival_time: u32,
    service_duration: u32,
}

/// The event to process at a particular simulation time.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum EventKind {
    Departure,
    Arrival { service_duration: u32 },
}

/// A queue arrival or service completion in the event schedule.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Event {
    time: u32,
    priority: u8,
    customer_id: usize,
    kind: EventKind,
}

/// The number of customers served and their mean time waiting for service.
#[derive(Debug, PartialEq)]
struct QueueReport {
    served: usize,
    average_wait_seconds: f64,
}

/// Simulates fixed customer arrivals and service times with a min-heap event
/// queue.
fn simulate_single_server_queue() -> QueueReport {
    const ARRIVAL_TIMES: [u32; 5] = [0, 1, 2, 13, 14];
    const SERVICE_DURATIONS: [u32; 5] = [3, 2, 1, 1, 1];

    let mut events = BinaryHeap::new();
    for (customer_id, (time, service_duration)) in
        ARRIVAL_TIMES.into_iter().zip(SERVICE_DURATIONS).enumerate()
    {
        events.push(Reverse(Event {
            time,
            priority: 1,
            customer_id,
            kind: EventKind::Arrival { service_duration },
        }));
    }

    let mut waiting = VecDeque::new();
    let mut server_busy = false;
    let mut served = 0;
    let mut total_wait_seconds = 0_u64;

    while let Some(Reverse(event)) = events.pop() {
        match event.kind {
            EventKind::Arrival { service_duration } => {
                let customer = Customer {
                    id: event.customer_id,
                    arrival_time: event.time,
                    service_duration,
                };
                if server_busy {
                    waiting.push_back(customer);
                } else {
                    server_busy = true;
                    events.push(Reverse(Event {
                        time: event.time + customer.service_duration,
                        priority: 0,
                        customer_id: customer.id,
                        kind: EventKind::Departure,
                    }));
                }
            }
            EventKind::Departure => {
                served += 1;
                if let Some(customer) = waiting.pop_front() {
                    total_wait_seconds +=
                        u64::from(event.time - customer.arrival_time);
                    events.push(Reverse(Event {
                        time: event.time + customer.service_duration,
                        priority: 0,
                        customer_id: customer.id,
                        kind: EventKind::Departure,
                    }));
                } else {
                    server_busy = false;
                }
            }
        }
    }

    let average_wait_seconds = if served == 0 {
        0.0
    } else {
        total_wait_seconds as f64 / served as f64
    };
    QueueReport {
        served,
        average_wait_seconds,
    }
}

/// Runs the queue simulation and prints its summary.
fn main() {
    let report = simulate_single_server_queue();
    println!(
        "Served {} customers; average wait {:.1} seconds",
        report.served, report.average_wait_seconds
    );
}
// ANCHOR_END: example

/// Runs the discrete-event queue example through its entry point.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::main;
    use super::simulate_single_server_queue;

    /// Verifies that all customers are served with the expected mean queue
    /// wait.
    #[test]
    fn queue_serves_all_customers_with_expected_average_wait() {
        let report = simulate_single_server_queue();

        assert_eq!(report.served, 5);
        assert!((report.average_wait_seconds - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
