use std::path;
use std::convert;

#[cfg(test)]
fn get_config_directory<P: convert::AsRef<path::Path>>(test_name: P) -> path::PathBuf {
    path::Path::new(file!()).parent().unwrap().join("build").join(test_name)
}

mod io_error {
    #[test]
    fn cannot_run_build_command_in_empty_directory() {
        let empty_directory = super::get_config_directory("empty");
        let mut program = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("program");
        program
            .args(&["build", empty_directory.to_str().unwrap()])
            .assert()
            .failure();
    }
    
    #[test]
    fn cannot_run_build_command_since_unable_to_read_non_config_file() {
        let non_config_directory = super::get_config_directory("non-config");
        let mut program = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("program");
        program
            .args(&["build", non_config_directory.to_str().unwrap()])
            .assert()
            .failure();
    }
}

mod deserialize_error {
    #[test]
    fn cannot_deserialize_image_which_has_invalid_image_name() {
        let invalid_image_name_config_directory = super::get_config_directory("invalid-image-name");
        let mut program = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("program");
        program
            .args(&["build", invalid_image_name_config_directory.to_str().unwrap()])
            .assert()
            .failure();
    }
}

#[test]
fn build_minimum_set() {
    let minimum_set_directory = get_config_directory("minimum-set");
    let stdout = format!(r#"---
image:
  - scripts: []
    base_image:
      name: scratch
    name: image
    tag: latest

"#);
    let mut program = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("program");
    program
        .args(&["build", minimum_set_directory.to_str().unwrap()])
        .assert()
        .success()
        .stdout(stdout);
}
