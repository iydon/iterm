use std::env::var_os;
use std::ffi::OsString;
use std::process::exit;

use duct::cmd;

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
        return cmd!(&self.program, "has-session", "-t", name)
            .stdout_null()
            .run()
            .is_ok();
    }

    fn attach(&self, name: &String) {
        cmd!(&self.program, "attach-session", "-t", name)
            .run()
            .unwrap();
    }

    fn create(&self, name: &String) {
        cmd!(&self.program, "new-session", "-s", name)
            .run()
            .unwrap();
    }

    fn list(&self) {
        let output = cmd!(&self.program, "list-sessions")
            .unchecked()
            .run()
            .unwrap();
        exit(output.status.code().unwrap());
    }
}
