use clap::{arg, Command};

use dmg_helper::temp_dir;
use spinners::{Spinner, Spinners};
use std::fs::remove_file;
use std::path::Path;
use std::process::exit;

use crate::seed::{load_seed, Seed};

pub mod common;
pub mod dmg_helper;
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

fn main() -> Result<(), ureq::Error> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let input: &str = sub_matches.get_one::<String>("PACKAGE").expect("required");

            // Get and load seed
            let mut sp = Spinner::new(Spinners::Dots, "Fetching seed".into());
            let body: String = ureq::get(&format!("http://0.0.0.0:3000/seed?id={input}"))
                .call()?
                .into_string()?;
            let seed: Seed = load_seed(&body);
            sp.stop_and_persist("\x1b[32m✔\x1b[0m", "Fetched seed".into());

            // Seed compatibility check
            let mut sp = Spinner::new(Spinners::Dots, "Checking seed".into());
            if Path::new(&format!("/Applications/{}", seed.app_name)).exists() {
                sp.stop_and_persist(
                    "\x1b[32m✖\x1b[0m",
                    "The application is already installed".into(),
                );
                exit(1);
            } else {
                sp.stop_and_persist("\x1b[32m✔\x1b[0m", "Seed is ready".into());
            }
            // let os = common::macos_version(); // TODO: Check if macOS version is compatible

            // Install seed
            let mut sp = Spinner::new(Spinners::Dots, format!("Installing {}", seed.name));
            // let arch = common::architecture();
            dmg_helper::download_dmg(&common::get_download_url(&seed));
            let path_to_app = dmg_helper::get_app_path(
                &format!("{}/app.dmg", dmg_helper::temp_dir()),
                &seed.app_name,
            );
            let ret = install::install_app(&path_to_app.1, &seed.app_name);
            if ret.0 != 0 {
                sp.stop_and_persist(
                    "\x1b[32m✖\x1b[0m",
                    format!("Failed to install {}: {}", seed.name, ret.1),
                );
                exit(ret.0);
            } else {
                sp.stop_and_persist("\x1b[32m✔\x1b[0m", format!("Installed {}", seed.name));
                install::xattr(&format!("/Applications/{}", seed.app_name));
            }
        }

        _ => unreachable!(),
    }

    // Clean up
    let mut sp = Spinner::new(Spinners::Dots, "Cleaning up".into());
    dmg_helper::detach(&dmg_helper::mount_dir());
    match remove_file(format!("{}/app.dmg", temp_dir())) {
        Ok(()) => (),
        Err(_) => (),
    }
    sp.stop_and_persist("\x1b[32m✔\x1b[0m", "Done!".into());
    exit(0);
}
