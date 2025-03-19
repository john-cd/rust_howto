// ANCHOR: example
use std::cmp::Reverse;

use priority_queue::PriorityQueue;

// Add this dependency to your `Cargo.toml`:
// [dependencies]
// priority-queue = "1.3.1"

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Task {
    id: u32,
    name: String,
}

impl Task {
    fn new(id: u32, name: &str) -> Self {
        Task {
            id,
            name: name.to_string(),
        }
    }
}

fn main() {
    println!("===== Basic Priority Queue Usage =====");

    // Create a new priority queue where higher values have higher priority
    let mut pq = PriorityQueue::new();

    // Insert elements with priorities
    pq.push("high priority task", 10);
    pq.push("medium priority task", 5);
    pq.push("low priority task", 1);

    // Pop elements in order of priority
    while let Some((task, priority)) = pq.pop() {
        println!("Task: {}, Priority: {}", task, priority);
    }

    println!("\n===== Min Priority Queue (using Reverse) =====");

    // Create a min priority queue using `Reverse`.
    let mut min_pq = PriorityQueue::new();

    // Lower values will have higher priority using `Reverse`.
    min_pq.push("urgent task", Reverse(1));
    min_pq.push("normal task", Reverse(5));
    min_pq.push("low urgency task", Reverse(10));

    while let Some((task, Reverse(priority))) = min_pq.pop() {
        println!("Task: {}, Priority: {}", task, priority);
    }

    println!("\n===== Priority Queue with Custom Types =====");

    let mut task_queue = PriorityQueue::new();

    // Add tasks with custom priorities
    task_queue.push(Task::new(1, "Fix critical bug"), 100);
    task_queue.push(Task::new(2, "Update documentation"), 30);
    task_queue.push(Task::new(3, "Refactor code"), 50);
    task_queue.push(Task::new(4, "Implement new feature"), 80);

    // Print queue information
    println!("Queue size: {}", task_queue.len());
    println!("Is empty: {}", task_queue.is_empty());

    // Get highest priority task without removing it
    if let Some((task, priority)) = task_queue.peek() {
        println!(
            "Highest priority task: {:?} with priority {}",
            task, priority
        );
    }

    // Change priority of a task
    let task_to_change = Task::new(2, "Update documentation");
    if let Some(old_priority) = task_queue.change_priority(&task_to_change, 75)
    {
        println!(
            "Changed priority of task {:?} from {} to 75",
            task_to_change, old_priority
        );
    }

    // Process all tasks
    println!("\nProcessing tasks in priority order:");
    while let Some((task, priority)) = task_queue.pop() {
        println!("Processing: {:?} (Priority: {})", task, priority);
    }

    println!("\n===== Advanced Usage =====");

    let mut advanced_pq = PriorityQueue::new();

    // Push multiple items
    advanced_pq.push("Task A", 5);
    advanced_pq.push("Task B", 3);
    advanced_pq.push("Task C", 7);

    // Get priority of an item
    if let Some(priority) = advanced_pq.get_priority(&"Task B") {
        println!("Priority of Task B: {}", priority);
    }

    // Increase the priority of an item
    if let Some(old_priority) = advanced_pq.push_increase("Task B", 8) {
        println!("Increased priority from {} to 8", old_priority);
    }

    // Iterate through the queue without consuming it (not sorted)
    println!("\nAll items in queue:");
    for (item, priority) in advanced_pq.iter() {
        println!("{}: {}", item, priority);
    }

    // Convert to a vector and sort by priority
    let items_vec: Vec<_> = advanced_pq.into_sorted_vec();
    println!("\nSorted items:");
    for item in items_vec {
        println!("{}", item);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
