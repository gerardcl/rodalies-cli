#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command; // Run programs

    #[test]
    fn cli_fails_when_defaults_but_no_interactive_input_provided(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rodalies-cli").unwrap();

        cmd.assert().failure().stderr(predicate::str::contains(
            "Please, provide at least 3 characters of the station name",
        ));

        Ok(())
    }

    #[test]
    fn cli_fails_when_only_one_station_point_provided() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rodalies-cli").unwrap();

        cmd.arg("-f 12")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "Please, specify origin and destination station IDs",
            ));

        Ok(())
    }

    #[test]
    fn cli_success_when_searching_with_value() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rodalies-cli").unwrap();

        cmd.arg("-ssils")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Searching stations that contain the text: 'sils'",
            ));

        Ok(())
    }
}
