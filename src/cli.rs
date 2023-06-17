use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("acai")
        .about("Fast package manager for macOS apps")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("install")
                .about("Installs packages")
                .arg(arg!(<PACKAGE> "Package to install"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("search")
                .about("Searches seeds")
                .arg(arg!(<QUERY> "Search query").required(false)),
        )
}
