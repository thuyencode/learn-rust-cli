#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use std::error;
    use std::process::Command;

    const EXAMPLE_TEXT: &str = "A test\nActual content\nMore content\nAnother test";

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn error::Error>> {
        let mut cmd = Command::cargo_bin("grrs")?;

        cmd.arg("foobar").arg("test/file/doesnt/exist");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("could not read file"));

        Ok(())
    }

    #[test]
    fn find_content_in_file() -> Result<(), Box<dyn error::Error>> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str(EXAMPLE_TEXT)?;

        let mut cmd = Command::cargo_bin("grrs")?;

        cmd.arg("test").arg(file.path());
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("A test\nAnother test"));

        Ok(())
    }

    #[test]
    fn print_file_content_when_pattern_is_an_empty_string() -> Result<(), Box<dyn error::Error>> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str(EXAMPLE_TEXT)?;

        let mut cmd = Command::cargo_bin("grrs")?;

        cmd.arg("").arg(file.path());
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(EXAMPLE_TEXT));

        Ok(())
    }
}
