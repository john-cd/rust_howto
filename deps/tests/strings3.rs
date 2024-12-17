#![allow(clippy::vec_init_then_push)]
#![allow(clippy::useless_vec)]
// ANCHOR: example

#[macro_use(concat_string)]
extern crate concat_string;

#[macro_use(concat_strs)]
extern crate concat_strs;

static DATE: &str = "2024-01-15";
static T: &str = "T";
static TIME: &str = "12:00:09Z";

fn main() {
    let _datetime = &[DATE, T, TIME].concat();

    let _datetime = &[DATE, TIME].join(T);

    let _datetime = &[DATE, T, TIME].join("");

    let _datetime = &[DATE, T, TIME].join("");

    let list = [DATE, T, TIME];
    // let _datetime: String = list.iter().map(|x| *x).collect();
    let _datetime: String = list.iter().copied().collect();

    let list = vec![DATE, T, TIME];
    // let _datetime: String = list.iter().map(|x| *x).collect();
    let _datetime: String = list.iter().copied().collect();

    let _datetime = &format!("{}{}{}", DATE, T, TIME);

    let _datetime = &format!("{DATE}{T}{TIME}");

    let mut datetime = String::new();
    datetime.push_str(DATE);
    datetime.push_str(T);
    datetime.push_str(TIME);

    let mut datetime = Vec::<String>::new();
    datetime.push(String::from(DATE));
    datetime.push(String::from(T));
    datetime.push(String::from(TIME));
    let _datetime = datetime.join("");

    let mut datetime = String::with_capacity(20);
    datetime.push_str(DATE);
    datetime.push_str(T); // or 'T'
    datetime.push_str(TIME);

    let _datetime =
        &(String::from(DATE) + &String::from(T) + &String::from(TIME));

    let _datetime = &(String::from(DATE) + T + TIME);

    let _datetime = &(DATE.to_owned() + T + TIME);

    let _datetime = &(DATE.to_string() + T + TIME);

    let _datetime = concat_string!(DATE, T, TIME);

    let datetime = &concat_strs!(DATE, T, TIME);
    
    println!("{}", datetime);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
