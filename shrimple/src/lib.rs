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
        Ok(self
            .shrimp_stdout()?
            .into_iter()
            .group_by(|elt| *elt != '\n' as u8)
            .into_iter()
            .filter_map(|(b, v)| b.then(|| v.map(|c| c as char).collect::<String>()))
            .collect::<Vec<String>>())
    }
    fn shrimp_stdout(&mut self) -> io::Result<Vec<u8>> {
        Ok(self.output()?.stdout)
    }

    fn shrimp_piped(&mut self) -> io::Result<Child> {
        self.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()
    }
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
