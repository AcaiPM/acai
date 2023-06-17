use dmg_helper::temp_dir;
use spinners::{Spinner, Spinners};
use std::fs::remove_file;
use std::path::Path;
use std::process::ExitCode;

use crate::seed::{load_seed, Seed};

pub mod cli;
pub mod common;
pub mod dmg_helper;
pub mod install;
pub mod seed;

fn main() -> Result<ExitCode, ureq::Error> {
    let matches = cli::cli().get_matches();

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
                return Ok(ExitCode::FAILURE);
            } else {
                sp.stop_and_persist("\x1b[32m✔\x1b[0m", "Seed is ready".into());
            }
            // let os = common::macos_version(); // TODO: Check if macOS version is compatible

            // Install seed
            let mut sp = Spinner::new(Spinners::Dots, format!("Installing {}", seed.name));
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
                return Ok(ExitCode::FAILURE);
            } else {
                sp.stop_and_persist("\x1b[32m✔\x1b[0m", format!("Installed {}", seed.name));
                install::xattr(&format!("/Applications/{}", seed.app_name));
            }

            // Clean up
            let mut sp = Spinner::new(Spinners::Dots, "Cleaning up".into());
            dmg_helper::detach(&dmg_helper::mount_dir());
            match remove_file(format!("{}/app.dmg", temp_dir())) {
                Ok(()) => (),
                Err(_) => (),
            }
            sp.stop_and_persist("\x1b[32m✔\x1b[0m", "Done!".into());
        }

        Some(("search", sub_matches)) => {
            if sub_matches.get_one::<String>("QUERY").is_some() {
                let input: &str = sub_matches.get_one::<String>("QUERY").expect("required");
                let body: String = ureq::get(&format!("http://0.0.0.0:3000/search?id={input}"))
                    .call()?
                    .into_string()?;

                println!("\x1b[01m\x1b[04mSearch results:\x1b[0m");
                let v: seed::Search = serde_json::from_str(&body).unwrap();
                let seeds = v.seeds;
                for seed in seeds {
                    println!("{}", seed);
                }
            } else {
                let body: String = ureq::get(&format!("http://0.0.0.0:3000/search"))
                    .call()?
                    .into_string()?;

                println!("\x1b[01m\x1b[04mAvailable seeds:\x1b[0m");
                let v: seed::Search = serde_json::from_str(&body).unwrap();
                let seeds = v.seeds;
                for seed in seeds {
                    println!("{}", seed);
                }
            }
        }

        _ => unreachable!(),
    }

    return Ok(ExitCode::SUCCESS);
}
