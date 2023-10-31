use std::error::Error;

use clap::{arg, Command};
use duct::cmd;

const SCREEN: &str = "screen";
const TMUX: &str = "tmux";
const ZELLIJ: &str = "zellij";

fn main() -> Result<(), Box<dyn Error>> {
    let command = Command::new("iterm")
        .version("0.1.0")
        .arg_required_else_help(true)
        .subcommand(Command::new(SCREEN).arg(arg!([NAME])))
        .subcommand(Command::new(TMUX).arg(arg!([NAME])))
        .subcommand(Command::new(ZELLIJ).arg(arg!([NAME])));
    match command.get_matches().subcommand() {
        Some((SCREEN, sub_matches)) => match sub_matches.get_one::<String>("NAME") {
            Some(name) => match cmd!(SCREEN, "-ls", name).stdout_null().run() {
                Ok(_) => cmd!(SCREEN, "-r", name).run()?,
                Err(_) => cmd!(SCREEN, "-S", name).run()?,
            },
            None => cmd!(SCREEN, "-ls").run()?,
        },
        Some((TMUX, sub_matches)) => match sub_matches.get_one::<String>("NAME") {
            Some(name) => match cmd!(TMUX, "has-session", "-t", name).stdout_null().run() {
                Ok(_) => cmd!(TMUX, "attach-session", "-t", name).run()?,
                Err(_) => cmd!(TMUX, "new-session", "-s", name).run()?,
            },
            None => cmd!(TMUX, "list-sessions").run()?,
        },
        Some((ZELLIJ, sub_matches)) => match sub_matches.get_one::<String>("NAME") {
            Some(name) => cmd!(ZELLIJ, "--session", name)
                .run()
                .or_else(|_| cmd!(ZELLIJ, "attach", name).run())?,
            None => cmd!(ZELLIJ, "list-sessions").run()?,
        },
        _ => unreachable!(),
    };
    return Ok(());
}
