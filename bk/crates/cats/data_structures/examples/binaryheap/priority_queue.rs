#![allow(dead_code)]
// ANCHOR: example
//! Add this dependency to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! priority-queue = "1.3.1" # Or latest.
//! ```

use std::cmp::Reverse;

use priority_queue::PriorityQueue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Task {
    id: u32,
    name: String,
}

impl Task {
    /// Creates a new `Task` instance.
    fn new(id: u32, name: &str) -> Self {
        Task {
            id,
            name: name.to_string(),
        }
    }
}

fn main() {
    println!("Basic Priority Queue Usage:");

    // Create a new priority queue where higher values have higher priority:
    let mut pq = PriorityQueue::new();

    // Insert elements with priorities:
    pq.push("high priority task", 10);
    pq.push("low priority task", 1);
    pq.push("medium priority task", 5);
    // If an element equal to item is already in the queue,
    // its priority is updated and the old priority is returned:
    assert_eq!(pq.push("medium priority task", 3), Some(5));

    // Pop elements in order of priority:
    while let Some((task, priority)) = pq.pop() {
        println!("Task: {task}, Priority: {priority}");
    }

    println!("\nMin Priority Queue:");

    // Create a min priority queue using `Reverse`.
    let mut min_pq = PriorityQueue::new();

    // Lower values will have higher priority using `Reverse`:
    min_pq.push("urgent task", Reverse(1));
    min_pq.push("normal task", Reverse(5));
    min_pq.push("low urgency task", Reverse(10));

    while let Some((task, Reverse(priority))) = min_pq.pop() {
        println!("Task: {task}, Priority: {priority}");
    }

    println!("\nPriority Queue with a Custom Type:");

    let mut task_queue = PriorityQueue::new();

    // Add tasks with custom priorities:
    task_queue.push(Task::new(1, "Fix critical bug"), 100);
    task_queue.push(Task::new(2, "Update documentation"), 30);
    task_queue.push(Task::new(3, "Refactor code"), 50);
    task_queue.push(Task::new(4, "Implement new feature"), 80);

    // Print queue information:
    println!("Queue size: {}", task_queue.len());
    println!("Is empty: {}", task_queue.is_empty());

    // Get the highest priority task without removing it:
    if let Some((task, priority)) = task_queue.peek() {
        println!("Highest priority task: {task:?} with priority {priority}",);
    }

    // Change the priority of a task:
    let task_to_change = Task::new(2, "Update documentation");
    if let Some(old_priority) = task_queue.change_priority(&task_to_change, 75)
    {
        println!(
            "Changed priority of task {task_to_change:?} from {old_priority} to 75",
        );
    }

    // Process all tasks:
    println!("\nProcessing tasks in priority order:");
    while let Some((task, priority)) = task_queue.pop() {
        println!("Processing: {task:?} (Priority: {priority})");
    }

    println!("\nAdvanced Usage:");

    let mut advanced_pq = PriorityQueue::new();

    // Push multiple items:
    advanced_pq.push("Task A", 5);
    advanced_pq.push("Task B", 3);
    advanced_pq.push("Task C", 7);

    // Get priority of an item:
    if let Some(priority) = advanced_pq.get_priority(&"Task B") {
        println!("Priority of Task B: {priority}");
    }

    // Increase the priority of an item, or insert it if not present.
    // If an element equal to item is already in the queue with an equal or
    // higher priority, its priority is not changed.
    if let Some(old_priority) = advanced_pq.push_increase("Task B", 8) {
        println!("Increased priority from {old_priority} to 8");
    }

    // Iterate through the queue without consuming it (not sorted):
    println!("\nAll items in queue:");
    for (item, priority) in advanced_pq.iter() {
        println!("{item}: {priority}");
    }

    // Convert to a vector and sort by priority:
    let items_vec: Vec<_> = advanced_pq.into_sorted_vec();
    println!("\nSorted items:");
    for item in items_vec {
        println!("{item}");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
