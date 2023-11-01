#[macro_export]
macro_rules! run {
    ( $program:expr $(, $arg:expr )* $(,)? ) => {
        {
            use std::ffi::OsString;
            use std::process::Command;

            let args: std::vec::Vec<OsString> = std::vec![$( Into::<OsString>::into($arg) ),*];
            Command::new($program).args(args)
        }
    };
}

pub trait Term {
    fn default() -> Self;

    fn exists(&self, name: &String) -> bool;
    fn attach(&self, name: &String);
    fn create(&self, name: &String);
    fn list(&self);

    fn api(&self, arg: Option<&String>) {
        return match arg {
            Some(name) => match self.exists(name) {
                true => self.attach(name),
                false => self.create(name),
            },
            None => self.list(),
        };
    }
}
