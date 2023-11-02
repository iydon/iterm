mod app;
mod config;
mod util;

use clap::{arg, Command};

use crate::app::{screen as S, tmux as T, zellij as Z};
use crate::util::Term;

fn main() {
    let command = Command::new(config::NAME)
        .about(config::ABOUT)
        .version(config::VERSION)
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
        .subcommand(Command::new(S::NAME).alias(S::ALIAS).arg(arg!([NAME])))
        .subcommand(Command::new(T::NAME).alias(T::ALIAS).arg(arg!([NAME])))
        .subcommand(Command::new(Z::NAME).alias(Z::ALIAS).arg(arg!([NAME])));
    match command.get_matches().subcommand() {
        Some((key @ (S::NAME | T::NAME | Z::NAME), sub_matches)) => {
            let name = sub_matches.get_one::<String>("NAME");
            match key {
                S::NAME => S::App::default().api(name),
                T::NAME => T::App::default().api(name),
                Z::NAME => Z::App::default().api(name),
                _ => unreachable!(),
            };
        }
        Some(("bash", _)) => {
            // eval "$(iterm bash)"
            vec![
                (S::NAME, S::ALIAS),
                (T::NAME, T::ALIAS),
                (Z::NAME, Z::ALIAS),
            ]
            .into_iter()
            .for_each(|(name, alias)| {
                println!("alias it{}='{} {name}'", alias, config::NAME);
            });
        }
        _ => unreachable!(),
    };
}
