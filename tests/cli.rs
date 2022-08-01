#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command; // Run programs

    #[test]
    fn cli_call_no_interactive_without_args() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rodalies-cli").unwrap();
        cmd.arg("--from=0");

        cmd.assert().failure().stderr(predicate::str::contains(
            "Please, specify origin and destination station IDs",
        ));

        Ok(())
    }
}
