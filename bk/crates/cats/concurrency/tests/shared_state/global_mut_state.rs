// ANCHOR: example
use std::sync::Mutex;

use anyhow::Result;
use anyhow::anyhow;
use lazy_static::lazy_static;

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT
        .lock()
        .map_err(|_| anyhow!("Failed to acquire MutexGuard"))?;
    db.push(fruit.to_string());
    Ok(())
}

fn main() -> Result<()> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT
            .lock()
            .map_err(|_| anyhow!("Failed to acquire MutexGuard"))?;

        db.iter().enumerate().for_each(|(i, item)| {
            println!("{}: {}", i, item);
        });
    }
    insert("grape")?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [review lazy_constant.rs and lazy_static.rs and global_mut_state.rs](https://github.com/john-cd/rust_howto/issues/939)
