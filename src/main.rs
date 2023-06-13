use clap::{arg, Command};

use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

pub mod dmg_helper;
pub mod common;
pub mod install;
pub mod seed;

fn cli() -> Command {
    Command::new("acai")
        .about("Fast package manager for macOS apps")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("install")
                .about("Installs packages")
                .arg(arg!(<PACKAGE> "The package to install"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let mut text: String = "Installing ".to_owned();
            let text2: &str = sub_matches.get_one::<String>("PACKAGE").expect("required");
            text.push_str(text2);

            let mut sp = Spinner::new(Spinners::Dots, text.into());
            sleep(Duration::from_secs(3));
            sp.stop();
            println!(""); // idk newline or something lol
        }

        _ => unreachable!(),
    }


    // for testing purposes below !!
    let path_to_app = dmg_helper::get_app_path("~/Downloads/googlechrome.dmg", "Google Chrome.app");
    println!("{}", common::to_str(path_to_app));

    let chrome_seed = seed::load_seed("data/seeds/google-chrome.json");
    println!("{:?}", chrome_seed.downloads);

    let arch = common::architecture();
    println!("{}", common::to_str(arch));

    let os = common::macos_version();
    println!("{}", common::to_str(os));
}
