use anyhow::Result;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(&pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::find_matches;

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        find_matches("i am a banana\ni am an orange", "banana", &mut result)
            .expect("error encountered");
        assert_eq!(result, b"i am a banana\n");
    }
}
