#![allow(clippy::vec_init_then_push)]

#[test]
fn test() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);

    let mut v = vec![1, 2, 3]; // or vec!(1, 2, 3)

    let _third: &i32 = &v[2]; // read

    let _third: Option<&i32> = v.get(2);

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50; // dereference operator
    }
}
