#![allow(unused_imports)]
use shrimple::{Shrimpipe, Shrimple, Shrimpout};
use std::io::Write;
use std::process::Command;

fn main() {
    let file_name = Command::new("dmenu")
        .args([
            "-l",
            "20",
            "-sb",
            "#a62ca6",
            "-i",
            "-p",
            "Anki Screenshot Name ",
            "-fn",
            "JetBrains Mono:size=10:style=Bold:antialias=true:autohint=true",
        ])
        .shrimp_vec()
        .unwrap();
    let Some(file_name) = file_name.first() else { return; };
    let xdg_data_home = std::env!("XDG_DATA_HOME");

    let file_path = format!("{xdg_data_home}/Anki2/User 1/collection.media/{file_name}.png");
    let output = Command::new("maim")
        .args(["-u", "-s", &file_path])
        .shrimp_exec()
        .unwrap();

    if output.success() {
        Command::new("notify-send")
            .args(["Screenshot", &format!("Saved to {file_path}")])
            .shrimp_exec()
            .unwrap();
        Command::new("xsel")
            .args(["-ib"])
            .stdin_write(format!("<img src={file_name}.png>").as_bytes())
            .and_then(|mut v| v.wait())
            .unwrap();
    } else {
        Command::new("notify-send")
            .args(["Screenshot", &format!("Something went wrong...")])
            .shrimp_exec()
            .unwrap();
    }
}
