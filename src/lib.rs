use anyhow::Context;
use std::io;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            let _ = writeln!(writer, "{}", line).with_context(|| "error while reading".to_string());
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();

    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);

    assert_eq!(result, b"lorem ipsum\n");
}
