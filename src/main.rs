mod cli;
mod downloader;
mod remover;

use downloader::download_and_install;
use remover::remove_package;

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {
        let package = matches.get_one::<String>("package").unwrap();
        // let repo = matches.get_one::<String>("repo");
        let version = matches.get_one::<String>("version");
        if version == None {
            download_and_install(package.to_string(), None);
        } else {
            download_and_install(package.to_string(), version.cloned());
        }
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        let package = matches.get_one::<String>("package").unwrap();
        remove_package(package.to_string());
        
        
    }
    
    if let Some(matches) = matches.subcommand_matches("upgrade") {
        let package = matches.get_one::<String>("package").unwrap();
        let repo = matches.get_one::<String>("repo");
        
        if repo == None || repo.unwrap() == "*" {
            println!("Upgrading package: {} from all repos", package);
        } else {
            println!("Upgrading package: {} from repo: {}", package, repo.unwrap());
        }
    }
    
    if let Some(matches) = matches.subcommand_matches("search") {
        let package = matches.get_one::<String>("package").unwrap();
        let local = matches.get_flag("local");
        
        if local {
            println!("Searching for package: {} locally", package);
        } else {
            println!("Searching for package: {} in remote repos", package);
        }
    }
    
    if let Some(matches) = matches.subcommand_matches("sync") {
        let repo = matches.get_one::<String>("repo");
        
        if repo == None || repo.unwrap() == "*" {
            println!("Syncing with all repos");
        } else {
            println!("Syncing with repo: {}", repo.unwrap());
        }
    }
}
