use clap::{Parser, Subcommand};
mod commands;
use commands::{ install::InstallArgs, remove::RemoveArgs, search::SearchArgs, sync::SyncArgs, upgrade::UpgradeArgs };
mod downloader;
use downloader::download_and_install;

#[derive(Parser)]
#[command(name = "winch")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install(InstallArgs),
    Search(SearchArgs),
    Remove(RemoveArgs),
    Sync(SyncArgs),
    Upgrade(UpgradeArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install(mut args) => {
            if args.repo.is_none() {
                let new_repo = Some("index.winchteam.dev".to_string());
                args.repo = new_repo.clone();
            } else {
                args.repo = args.repo.clone();
            }
            download_and_install(args.package, None);
        },
        Commands::Search(args) => {
            unimplemented!();
        },
        Commands::Remove(args) => {
            unimplemented!();
        },
        Commands::Sync(mut args) => {
            if args.repo.is_none() {
                let new_repo = Some("*".to_string());
                args.repo = new_repo.clone();
            } else {
                args.repo = args.repo.clone();
            }
        },
        Commands::Upgrade(mut args) => {
            if args.repo.is_none() {
                let new_repo = Some("*".to_string()); // * means all repos
                args.repo = new_repo.clone();
            } else {
                args.repo = args.repo.clone();
            }
            if args.package.is_none() {
                let new_package = Some("*".to_string()); // * means all packages
                args.package = new_package.clone();
            } else {
                args.package = args.package.clone();
            }
        },
    }

}
