mod app;
mod config;
mod util;

use clap::{arg, Command};

use crate::app::{screen, tmux, zellij};
use crate::util::Term;

fn main() {
    let command = Command::new(config::NAME)
        .about(config::ABOUT)
        .version(config::VERSION)
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(screen::NAME)
                .alias(screen::ALIAS)
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new(tmux::NAME)
                .alias(tmux::ALIAS)
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new(zellij::NAME)
                .alias(zellij::ALIAS)
                .arg(arg!([NAME])),
        );
    match command.get_matches().subcommand() {
        Some((key @ (screen::NAME | tmux::NAME | zellij::NAME), sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME");
            match key {
                screen::NAME => screen::App::default().api(name),
                tmux::NAME => tmux::App::default().api(name),
                zellij::NAME => zellij::App::default().api(name),
                _ => unreachable!(),
            };
        }
        Some(("bash", _)) => {
            // eval "$(iterm bash)"
            let items = vec![
                (screen::NAME, screen::ALIAS),
                (tmux::NAME, tmux::ALIAS),
                (zellij::NAME, zellij::ALIAS),
            ];
            items.into_iter().for_each(|(name, alias)| {
                println!("alias it{}='{} {name}'", alias, config::NAME);
            });
        }
        _ => unreachable!(),
    };
}
