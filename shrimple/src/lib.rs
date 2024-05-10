use itertools::Itertools;
use std::io;
use std::io::Write;
use std::process::{Child, Command, ExitStatus, Stdio};

pub trait Shrimple {
    fn shrimp_exec(&mut self) -> io::Result<ExitStatus>;

    fn shrimp_spawn(&mut self) -> io::Result<Child>;

    fn shrimp_vec(&mut self) -> io::Result<Vec<String>>;

    fn shrimp_stdout(&mut self) -> io::Result<Vec<u8>>;

    fn shrimp_piped(&mut self) -> io::Result<Child>;
}

pub trait Shrimpipe<'a> {
    type ReturnChild;
    fn pipe(&mut self, other: &mut Command) -> io::Result<Child>;

    fn stdin_write(&'a mut self, bytes: &[u8]) -> io::Result<Self::ReturnChild>;
}

impl Shrimple for Command {
    fn shrimp_exec(&mut self) -> io::Result<ExitStatus> {
        self.status()
    }
    fn shrimp_spawn(&mut self) -> io::Result<Child> {
        self.spawn()
    }
    fn shrimp_vec(&mut self) -> io::Result<Vec<String>> {
        Ok(String::from_utf8(self.shrimp_stdout()?)
            .expect("filename must be utf8 encoded")
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect())
    }
    fn shrimp_stdout(&mut self) -> io::Result<Vec<u8>> {
        Ok(self.output()?.stdout)
    }

    fn shrimp_piped(&mut self) -> io::Result<Child> {
        self.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()
    }
}

#[test]
fn test_utf8_shrimp_vec() {
    let list = Command::new("ls")
        .args(["../test/utf8"])
        .shrimp_vec()
        .unwrap();
    let expected = [
        "Vinland Saga - 01 - ここではないどこか.ass",
        "Vinland Saga - 02 - 剣.ass",
        "Vinland Saga - 03 - 戦鬼.ass",
        "Vinland Saga - 04 - 本当の戦士.ass",
        "Vinland Saga - 05 - 戦鬼の子.ass",
        "Vinland Saga - 06 - 旅の始まり.ass",
        "Vinland Saga - 07 - 北人.ass",
        "Vinland Saga - 08 - 海の果ての果て.ass",
        "Vinland Saga - 09 - ロンドン橋の死闘.ass",
        "Vinland Saga - 10 - ラグナロク.ass",
        "Vinland Saga - 11 - 賭け.ass",
        "Vinland Saga - 12 - 対岸の国.ass",
        "Vinland Saga - 13 - 英雄の子.ass",
        "Vinland Saga - 14 - 暁光.ass",
        "Vinland Saga - 15 - 冬至祭のあと.ass",
        "Vinland Saga - 16 - ケダモノの歴史.ass",
        "Vinland Saga - 17 - 仕えし者.ass",
        "Vinland Saga - 18 - ゆりかごの外.ass",
        "Vinland Saga - 19 - 共闘.ass",
        "Vinland Saga - 20 - 王冠.ass",
        "Vinland Saga - 21 - 再会.ass",
        "Vinland Saga - 22 - 孤狼.ass",
        "Vinland Saga - 23 - 誤算.ass",
        "Vinland Saga - 24 - END OF THE PROLOGUE.ass",
    ];
    assert_eq!(list, expected);
}

pub trait Shrimpout {
    fn output_string(self) -> io::Result<Result<String, FromUtf8Error>>;

    fn stdout(self) -> io::Result<()>;
}

use std::string::FromUtf8Error;
impl Shrimpout for Child {
    fn output_string(self) -> io::Result<Result<String, FromUtf8Error>> {
        Ok(String::from_utf8(self.wait_with_output()?.stdout))
    }
    fn stdout(self) -> io::Result<()> {
        let output = self.wait_with_output()?;

        ::std::io::stdout().write_all(&output.stdout)
    }
}

impl<'a> Shrimpipe<'a> for Child {
    type ReturnChild = &'a mut Child;
    fn pipe(&mut self, other: &mut Command) -> io::Result<Child> {
        if let Some(stdout) = self.stdout.take() {
            other.stdin(stdout).stdout(Stdio::piped()).spawn()
        } else {
            other.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()
        }
    }
    fn stdin_write(&'a mut self, bytes: &[u8]) -> io::Result<Self::ReturnChild> {
        self.stdin.as_mut().unwrap().write_all(bytes).unwrap();
        Ok(self)
    }
}

impl<'a> Shrimpipe<'a> for Command {
    type ReturnChild = Child;
    /// Pipes output from self to input of child.
    fn pipe(&mut self, other: &mut Command) -> io::Result<Child> {
        self.shrimp_piped()?;
        if let Some(stdout) = self.spawn()?.stdout {
            other.stdin(stdout).stdout(Stdio::piped()).shrimp_spawn()
        } else {
            other.shrimp_piped()
        }
    }
    fn stdin_write(&mut self, bytes: &[u8]) -> io::Result<Self::ReturnChild> {
        let mut child = self.shrimp_piped().unwrap();

        child.stdin.as_mut().unwrap().write_all(bytes).unwrap();
        Ok(child)
    }
}
