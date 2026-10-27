// TODO Reviewed ??-??-2026
// TODO explain this is last resort

#![expect(dead_code, reason = "scratch example with unused items")]
// ANCHOR: example
//! Implement `AsRef` to "emulate inheritance".

// "Base type".
/// A "base type" that stores the common data of media files.
#[derive(Debug)]
struct MediaFile {
    /// The identifier.
    id: String,
    /// The name of the file.
    filename: String,
    /// The size of the file in bytes.
    file_size_bytes: u64,
    /// The tags.
    tags: Vec<String>,
}

// `ImageFile` is a specialized media file and acts as a "derived type".
/// A "derived type" that embeds a media file.
#[derive(Debug)]
struct ImageFile {
    // There is no 'is-a' inheritance in Rust. Use composition ('has-a').
    // The "base type" is simply embedded into the "derived type".
    /// The embedded media file.
    media: MediaFile,
    /// The width in pixels.
    width: u32,
    /// The height in pixels.
    height: u32,
    /// The model of the camera, if known.
    camera_model: Option<String>,
}

impl ImageFile {
    // The simplest way to access the nested data is, of course,
    // to write simple projection functions:
    /// Returns the size of the file in bytes.
    fn get_file_size_bytes(&self) -> u64 {
        self.media.file_size_bytes
    }
}

// In the (rare) cases where composition leads to writing a lot of boilerplate,
// consider implementing `AsRef` to allow `ImageFile` to be treated as a
// `&MediaFile`.
impl AsRef<MediaFile> for ImageFile {
    fn as_ref(&self) -> &MediaFile {
        &self.media
    }
}

// We also need to implement `AsRef` for the "base type":
impl AsRef<Self> for MediaFile {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// This function prints basic details of any media item.
/// It works with anything that can provide a cheap reference to a `MediaFile`.
/// Note that the conversion is fully _explicit_.
fn print_media_details<M: AsRef<MediaFile>>(item: M) {
    // Get the `&MediaFile` reference.
    let media_ref = item.as_ref();

    println!("ID: {}", media_ref.id);
    println!("Filename: {}", media_ref.filename);
    println!("Size: {} bytes", media_ref.file_size_bytes);
    println!("Tags: {:?}", media_ref.tags);
}

/// Creates an image file and prints the details of its media file.
fn main() {
    // Create an ImageFile.
    let photo = ImageFile {
        // Nested struct.
        media: MediaFile {
            id: "IMG_001".to_owned(),
            filename: "vacation_sunset.jpg".to_owned(),
            file_size_bytes: 2_500_000,
            tags: vec!["sunset".to_owned(), "vacation".to_owned()],
        },
        width: 1920,
        height: 1080,
        camera_model: Some("Canon EOS R6".to_owned()),
    };

    println!("Printing details for a photo:");
    // `&ImageFile` becomes `&MediaFile` via `AsRef`.
    print_media_details(&photo);

    // We can also pass a bare `MediaFile`:
    let bare_file = MediaFile {
        id: "TXT_001".to_owned(),
        filename: "notes.txt".to_owned(),
        file_size_bytes: 1024,
        tags: vec!["document".to_owned()],
    };
    println!("Printing details for a bare media file:");
    print_media_details(&bare_file);
}
// ANCHOR_END: example

/// Runs the AsRef example for the workspace test harness.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
