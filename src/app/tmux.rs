use std::env::var_os;
use std::ffi::OsString;
use std::process::{exit, Stdio};

use crate::run;
use crate::util::Term;

pub const NAME: &str = "tmux";
pub const ALIAS: &str = "t";
pub const ENV: &str = "ITERM_TMUX";

pub struct App {
    pub program: OsString,
}

impl Term for App {
    fn default() -> Self {
        let program = var_os(ENV).unwrap_or_else(|| NAME.into());
        return Self { program };
    }

    fn exists(&self, name: &String) -> bool {
        return run!(&self.program, "has-session", "-t", name)
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success();
    }

    fn attach(&self, name: &String) {
        run!(&self.program, "attach-session", "-t", name)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    fn create(&self, name: &String) {
        run!(&self.program, "new-session", "-s", name)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    fn list(&self) {
        let status = run!(&self.program, "list-sessions")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        exit(status.code().unwrap());
    }
}
