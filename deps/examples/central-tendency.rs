fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    let mean = match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    };

    println!("Mean of the data is {:?}", mean);
}
