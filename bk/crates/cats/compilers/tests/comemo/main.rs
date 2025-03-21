// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use comemo::{Tracked, Memoized};

// // A function we want to memoize
// fn expensive_function(n: u32) -> u32 {
//     println!("Calculating for {}", n);
//     // In a real scenario, this would be a complex calculation
//     n * 2
// }

// fn main() {
//     // Create a new memoization context.
//     let memo_context = Memoized::new();

//     // Tell comemo what input to track.
//     let tracked_input = Tracked::new(5);

//     // First call: The function will be executed.
//     let result1 = memo_context.memoize(||
// expensive_function(*tracked_input));     println!("Result 1: {}", result1);

//     // Second call with the same input: The memoized value will be returned.
//     let result2 = memo_context.memoize(||
// expensive_function(*tracked_input));     println!("Result 2: {}", result2);
// // "Calculating for 5" will NOT be printed

//     // Change the tracked input.
//     tracked_input.set(10);

//     // Third call with the new input: The function will be executed again.
//     let result3 = memo_context.memoize(||
// expensive_function(*tracked_input));     println!("Result 3: {}", result3);
// // "Calculating for 10" WILL be printed

//     // Example with a more complex struct:
//     #[derive(Debug, PartialEq, Eq, Clone, Copy)]
//     struct MyData {
//         a: u32,
//         b: String,
//     }

//     let tracked_data = Tracked::new(MyData { a: 20, b: "hello".to_string()
// });

//     let memo_context2 = Memoized::new();

//     let result4 = memo_context2.memoize(|| {
//         println!("Calculating with {:?}", *tracked_data);
//         tracked_data.a * 3
//     });
//     println!("Result 4: {}", result4);

//     // Modify a field in the tracked struct
//     let mut data = *tracked_data; // Get a copy of the data
//     data.a = 25;
//     tracked_data.set(data); // Set the new value

//     let result5 = memo_context2.memoize(|| {
//         println!("Calculating with {:?}", *tracked_data);
//         tracked_data.a * 3
//     });
//     println!("Result 5: {}", result5);

//     // Example with cloning to avoid mutable borrow issues.
//     // If you can't directly modify the Tracked value, you can clone it,
// modify the clone, and then set it.

//     let tracked_data2 = Tracked::new(MyData { a: 30, b: "world".to_string()
// });     let memo_context3 = Memoized::new();

//     let result6 = memo_context3.memoize(|| {
//         println!("Calculating with {:?}", *tracked_data2);
//         tracked_data2.a * 3
//     });
//     println!("Result 6: {}", result6);

//     let data2 = tracked_data2.get(); // Get a copy of the data
//     let mut data3 = data2.clone(); // Clone the copy
//     data3.a = 35;
//     tracked_data2.set(data3); // Set the new value

//     let result7 = memo_context3.memoize(|| {
//         println!("Calculating with {:?}", *tracked_data2);
//         tracked_data2.a * 3
//     });
//     println!("Result 7: {}", result7);

// }

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// TODO WIP finish NOW
