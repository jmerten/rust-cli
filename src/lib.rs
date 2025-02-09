use anyhow::Result;

pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
    json: bool,
) -> Result<()> {
    for line in content.lines() {
        if line.contains(&pattern) {
            if json {
                writeln!(
                    writer,
                    "{}",
                    serde_json::json!({"type":"match", "content": line})
                )?;
                continue;
            }
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::find_matches;

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        find_matches(
            "i am a banana\ni am an orange",
            "banana",
            &mut result,
            false,
        )
        .expect("error encountered");
        assert_eq!(result, b"i am a banana\n");
    }

    #[test]
    fn find_a_match_json() {
        let mut result = Vec::new();
        find_matches("i am a banana\ni am an orange", "banana", &mut result, true)
            .expect("error encountered");

        let expected = json!({"content":"i am a banana","type":"match"}).to_string();
        assert_eq!(
            String::from_utf8(result).unwrap(),
            format!("{}\n", expected)
        );
    }
}
