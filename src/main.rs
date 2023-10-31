use std::env::var_os;
use std::error::Error;

use clap::{arg, Command};
use duct::cmd;

const ITERM_NAME: &str = "iterm";
const ITERM_ABOUT: &str = "Terminal workspace (screen, tmux, zellij)";
const ITERM_VERSION: &str = "0.3.0";

const SCREEN: &str = "screen";
const TMUX: &str = "tmux";
const ZELLIJ: &str = "zellij";

fn main() -> Result<(), Box<dyn Error>> {
    let command = Command::new(ITERM_NAME)
        .about(ITERM_ABOUT)
        .version(ITERM_VERSION)
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
        .subcommand(Command::new(SCREEN).alias(&SCREEN[..1]).arg(arg!([NAME])))
        .subcommand(Command::new(TMUX).alias(&TMUX[..1]).arg(arg!([NAME])))
        .subcommand(Command::new(ZELLIJ).alias(&ZELLIJ[..1]).arg(arg!([NAME])));
    match command.get_matches().subcommand() {
        Some((SCREEN, sub_matches)) => {
            let program = var_os("ITERM_SCREEN").unwrap_or_else(|| SCREEN.into());
            match sub_matches.get_one::<String>("NAME") {
                Some(name) => match cmd!(&program, "-ls", name).stdout_null().run() {
                    Ok(_) => cmd!(program, "-r", name).run()?,
                    Err(_) => cmd!(program, "-S", name).run()?,
                },
                None => cmd!(program, "-ls").run()?,
            };
        }
        Some((TMUX, sub_matches)) => {
            let program = var_os("ITERM_TMUX").unwrap_or_else(|| TMUX.into());
            match sub_matches.get_one::<String>("NAME") {
                Some(name) => match cmd!(&program, "has-session", "-t", name)
                    .stdout_null()
                    .run()
                {
                    Ok(_) => cmd!(program, "attach-session", "-t", name).run()?,
                    Err(_) => cmd!(program, "new-session", "-s", name).run()?,
                },
                None => cmd!(program, "list-sessions").run()?,
            };
        }
        Some((ZELLIJ, sub_matches)) => {
            let program = var_os("ITERM_ZELLIJ").unwrap_or_else(|| ZELLIJ.into());
            match sub_matches.get_one::<String>("NAME") {
                Some(name) => cmd!(&program, "--session", name)
                    .run()
                    .or_else(|_| cmd!(program, "attach", name).run())?,
                None => cmd!(program, "list-sessions").run()?,
            };
        }
        Some(("bash", _)) => {
            // eval "$(iterm bash)"
            vec![SCREEN, TMUX, ZELLIJ].into_iter().for_each(|cmd| {
                println!("alias it{}='{ITERM_NAME} {cmd}'", &cmd[..1]);
            });
        }
        _ => unreachable!(),
    };
    return Ok(());
}
