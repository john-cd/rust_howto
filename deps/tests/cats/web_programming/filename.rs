// ANCHOR: example
use mime::Mime;

fn find_mimetype(filename: &str) -> Mime {
    let parts: Vec<&str> = filename.split('.').collect();

    let res = match parts.last() {
        Some(v) => match *v {
            "png" => mime::IMAGE_PNG,
            "jpg" => mime::IMAGE_JPEG,
            "json" => mime::APPLICATION_JSON,
            &_ => mime::TEXT_PLAIN,
        },
        None => mime::TEXT_PLAIN,
    };
    res
}

fn main() {
    let filenames = vec!["foobar.jpg", "foo.bar", "foobar.png"];
    for file in filenames {
        let mime = find_mimetype(file);
        println!("MIME for {}: {}", file, mime);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
