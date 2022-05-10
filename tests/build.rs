use std::convert;
use std::path;

#[cfg(test)]
fn get_config_directory<P: convert::AsRef<path::Path>>(test_name: P) -> path::PathBuf {
    path::Path::new(file!())
        .parent()
        .unwrap()
        .join("build")
        .join(test_name)
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
            .args(&[
                "build",
                invalid_image_name_config_directory.to_str().unwrap(),
            ])
            .assert()
            .failure();
    }
}

#[test]
fn build_minimum_set() {
    let minimum_set_directory = get_config_directory("minimum-set");
    let stdout = r#"---
image:
  - scripts: []
    base_image:
      name: scratch
    name: image
    tag: latest

"#
    .to_string();
    let mut program = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("program");
    program
        .args(&["build", minimum_set_directory.to_str().unwrap()])
        .assert()
        .success()
        .stdout(stdout);
}

#[test]
fn build_multi_image() {
    let config_directory = get_config_directory("multi-image");
    let stdout = r#"---
image:
  - scripts:
      - type: add
        source: "./source-file"
        destination: "./destination-file"
    base_image:
      name: scrach
      tag: latest
    name: image1
    tag: "22.04"
  - scripts:
      - type: add
        source: "./source-file"
        destination: /destination-file
    base_image:
      name: amethyst
      tag: "22.04"
    name: image2
    tag: latest

"#;
    let mut program = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("program");
    program
        .args(&["build", config_directory.to_str().unwrap()])
        .assert()
        .success()
        .stdout(stdout);
}
