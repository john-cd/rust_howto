// ANCHOR: example

use mime::Mime;

/// Find the mime type of a file based on its extension.
fn find_mimetype(filename: &str) -> Mime {
    // Split the filename by '.' and collect the parts into a vector.
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
