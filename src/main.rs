use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("pacman")
        .about("A package manager for all systems.")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Winch Team")

        .subcommand(
            Command::new("install")
                .short_flag('i')
                .long_flag("install")
                .about("Install packages.")

                .arg(
                    Arg::new("package")
                        .short('p')
                        .long("package")
                        .help("The package to install.")
                        .required(true)
                        .num_args(1),
                )
                .arg(
                    Arg::new("local")
                        .short('l')
                        .long("local")
                        .help("Install from local file or cache.")
                        .required(false)
                        .num_args(1),
                )
                .arg(
                    Arg::new("repo")
                        .short('r')
                        .long("repo")
                        .help("The repository to install from.")
                        .required(false)
                        .num_args(1),
                )
        )
        .subcommand(
            Command::new("search")
                .short_flag('s')
                .long_flag("search")
                .about("Query the package cache, local or remote.")
                .arg(
                    Arg::new("local")
                        .short('l')
                        .long("local")
                        .help("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("local")
                        .help("view package information")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("remove")
                .short_flag('r')
                .long_flag("remove")
                .about("Remove packages.")
                .arg(
                    Arg::new("package")
                        .short('p')
                        .long("package")
                        .help("The package to remove.")
                        .required(true)
                        .num_args(1),
                )
        )
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Synchronize packages from cache server.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .conflicts_with("info")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .help("search remote cache specifically for matching strings"),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .conflicts_with("search")
                        .short('i')
                        .action(ArgAction::SetTrue)
                        .help("view package information from remote cache"),
                )
        )
        .get_matches();
    }