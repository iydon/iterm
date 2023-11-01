use std::env::var_os;
use std::ffi::OsString;
use std::process::exit;

use duct::cmd;

use super::super::util::Term;

pub const NAME: &str = "zellij";
pub const ALIAS: &str = "z";
pub const ENV: &str = "ITERM_ZELLIJ";

pub struct App {
    pub program: OsString,
}

impl Term for App {
    fn default() -> Self {
        let program = var_os(ENV).unwrap_or_else(|| NAME.into());
        return Self { program };
    }

    fn exists(&self, _name: &String) -> bool {
        unimplemented!();
    }

    fn attach(&self, _name: &String) {
        unimplemented!();
    }

    fn create(&self, _name: &String) {
        unimplemented!();
    }

    fn list(&self) {
        unimplemented!();
    }

    fn api(&self, arg: Option<&String>) {
        match arg {
            Some(name) => {
                cmd!(&self.program, "--session", name)
                    .run()
                    .or_else(|_| cmd!(&self.program, "attach", name).run())
                    .unwrap();
            }
            None => {
                let output = cmd!(&self.program, "list-sessions")
                    .unchecked()
                    .run()
                    .unwrap();
                exit(output.status.code().unwrap());
            }
        };
    }
}
