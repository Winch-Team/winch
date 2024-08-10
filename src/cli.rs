use clap::{Arg, Command};

pub(crate) fn build_cli() -> Command {
    let matches = Command::new("winch")
        .version("0.1.0")
        .author("Winch Team")
        .about("The Winch package manager's CLI interface")
        .subcommand(
            Command::new("install")
                .about("install a package")
                .arg(Arg::new("package").required(true))
                .arg(Arg::new("version").short('v').long("version"))
                .arg(Arg::new("repo").short('r').long("repo"))
                .arg(Arg::new("local").short('l').long("local")),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a package that is installed")
                .arg(Arg::new("package").required(true)),
        )
        .subcommand(
            Command::new("upgrade")
                .about("upgrade package(s)")
                .arg(Arg::new("package").default_value("*"))
                .arg(Arg::new("repo").default_value("*").short('r').long("repo")),
        )
        .subcommand(
            Command::new("search")
                .about("search for a package")
                .arg(Arg::new("package").required(true))
                .arg(Arg::new("local").short('l').long("local").action(clap::ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("sync")
                .about("Sync your local cache with remote index servers")
                .arg(Arg::new("repo").default_value("*").short('r').long("repo")),
        );

    matches
}
