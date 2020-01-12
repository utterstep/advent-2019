#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_expected_output() {
        // assert that all existing solutions are valid after any change
        let output = Command::new("bash")
            .current_dir("../")
            .arg("./run_all.sh")
            .output()
            .unwrap();

        assert_eq!(output.stdout, include_bytes!("../reference.txt").to_vec(),);
    }
}
