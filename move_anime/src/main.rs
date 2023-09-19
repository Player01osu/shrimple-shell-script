use shrimple::{Shrimple, Shrimpout};
use std::process::Command;

fn main() {
    Command::new("head")
        .args(["-c", "10", "/dev/random"])
        .shrimp_piped()
        .unwrap()
        .stdout()
        .unwrap();
}
