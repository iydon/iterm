use std::env::var_os;
use std::ffi::OsString;
use std::process::{exit, Stdio};

use crate::run;
use crate::util::Term;

pub const NAME: &str = "screen";
pub const ALIAS: &str = "s";
pub const ENV: &str = "ITERM_SCREEN";

pub struct App {
    pub program: OsString,
}

impl Term for App {
    fn default() -> Self {
        let program = var_os(ENV).unwrap_or_else(|| NAME.into());
        return Self { program };
    }

    fn exists(&self, name: &String) -> bool {
        return run!(&self.program, "-ls", name)
            .stdout(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success();
    }

    fn attach(&self, name: &String) {
        run!(&self.program, "-r", name)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    fn create(&self, name: &String) {
        run!(&self.program, "-S", name)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    fn list(&self) {
        let status = run!(&self.program, "-ls").spawn().unwrap().wait().unwrap();
        exit(status.code().unwrap());
    }
}
