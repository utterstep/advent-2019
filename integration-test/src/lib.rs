#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_expected_output() {
        Command::new("cargo")
            .current_dir("../")
            .arg("build")
            .arg("--release")
            .status()
            .expect("failed to build");

        // assert that all existing solutions are valid after any change
        let output = Command::new("bash")
            .current_dir("../")
            .arg("./run_all.sh")
            .output()
            .expect("failed to run all solutions");

        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            include_str!("../reference.txt"),
        );
    }
}
