// use chrono::DateTime;
// use chrono::FixedOffset;
use chrono::Local;
// use chrono::Utc;

fn main() {
    let local_time = Local::now();
    // let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(),
    // Utc); let china_timezone = FixedOffset::east(8 * 3600);
    // let rio_timezone = FixedOffset::west(2 * 3600);
    println!("Local time now is {}", local_time);
    // println!("UTC time now is {}", utc_time);
    // println!(
    //     "Time in Hong Kong now is {}",
    //     utc_time.with_timezone(&china_timezone)
    // );
    // println!(
    //     "Time in Rio de Janeiro now is {}",
    //     utc_time.with_timezone(&rio_timezone)
    // );
}
