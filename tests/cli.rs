use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rust-cli")?;

        cmd.arg("foobar").arg("test/file/doesnt/exist");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("could not read file"));
        Ok(())
    }

    #[test]
    fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
        // create a testing file
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str("A test\nActual content\nMore content\nAnother test")?;

        let mut cmd = Command::cargo_bin("rust-cli")?;

        cmd.arg("test").arg(file.path());
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("A test\nAnother test"));
        
        Ok(())
    }
}
