import re

with open('mdbook-utils/src/markdown/extract_code.rs', 'r') as f:
    content = f.read()

test_mod = r"""
#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use rand::distr::{Alphanumeric, SampleString};

    fn create_temp_dir() -> PathBuf {
        let random_string = Alphanumeric.sample_string(&mut rand::rng(), 10);
        let mut path = std::env::temp_dir();
        path.push(format!("mdbook-utils-test-{}", random_string));
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn test_extract_code() {
        let temp_dir = create_temp_dir();
        let src_dir = temp_dir.join("src");
        let dest_dir = temp_dir.join("dest");
        fs::create_dir_all(&src_dir).unwrap();

        let md_content = "\
# Test file

Some text.

```rust
fn main() {
#   println!(\"hidden line\");
    println!(\"hello\");
}
```

More text.

```rust
fn second() {
    println!(\"world\");
}
```
";
        let md_file_path = src_dir.join("test_file.md");
        fs::write(&md_file_path, md_content).unwrap();

        extract_code_from_all_markdown_files_in(&src_dir, &dest_dir).unwrap();

        let rs_file_1 = dest_dir.join("test_file.rs");
        assert!(rs_file_1.exists());
        let content_1 = fs::read_to_string(rs_file_1).unwrap();
        // The implementation removes '# ' at the beginning of lines
        assert_eq!(content_1, "fn main() {\n  println!(\"hidden line\");\n    println!(\"hello\");\n}\n");

        let rs_file_2 = dest_dir.join("test_file1.rs");
        assert!(rs_file_2.exists());
        let content_2 = fs::read_to_string(rs_file_2).unwrap();
        assert_eq!(content_2, "fn second() {\n    println!(\"world\");\n}\n");

        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_remove_code() {
        let temp_dir = create_temp_dir();
        let src_dir = temp_dir.join("src");
        let dest_dir = temp_dir.join("dest");
        fs::create_dir_all(&src_dir).unwrap();

        let md_content = "\
# Test file

```rust
fn main() {}
```
";
        let md_file_path = src_dir.join("test_file.md");
        fs::write(&md_file_path, md_content).unwrap();

        remove_code_from_all_markdown_files_in(&src_dir, &dest_dir).unwrap();

        let result_content = fs::read_to_string(&md_file_path).unwrap();
        let expected_path = dest_dir.join("test_file.rs");

        // Note: remove_code_from_all_markdown_files_in replaces the code block with {#include <path>.rs}
        let expected_content = format!("\
# Test file

```rust
{{#include {}}}
```
", expected_path.display());

        assert_eq!(result_content, expected_content);

        fs::remove_dir_all(temp_dir).unwrap();
    }
}
"""

new_content = re.sub(r'#\[cfg\(test\)\].*', test_mod, content, flags=re.DOTALL)

with open('mdbook-utils/src/markdown/extract_code.rs', 'w') as f:
    f.write(new_content)

print("Replacement successful" if new_content != content else "Replacement failed")
