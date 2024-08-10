use colored::Colorize;
use std::{fs::remove_file, io::{self, Write}};
use term_kit::spinner::Spinner;

pub(crate) fn remove_package(package: String) {
    let home_dir = dirs::home_dir();
    let target_dir = home_dir.expect("No home dir found").join(".winch/bin/");
    let package = package.trim().to_string();

    if target_dir.join(format!("{}.json", package)).exists() {
        println!("{}\n", "Removing package:".green().bold());
        print!("{}", "Do you want to continue? (y/n) ".green().bold());


        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "Y" {
            let binary_name = read_sidecar_for_binary_name(package.to_owned());
            let spinner = Spinner::new("Removing package...".to_string());
            spinner.render();


            remove_file(format!("{}/{}", target_dir.display(), binary_name))
                .expect("Failed to remove binary");
            remove_file(format!("{}/{}.json", target_dir.display(), package))
                .expect("Failed to remove sidecar file");

            println!("{:?}", target_dir.join(format!("{}", binary_name)));
            spinner.stop();
            println!("{} {}\n", "Package removed: ".green().bold(), package);
        } else {
            println!("{}", "aborting removal!\n".red().bold());
        }
    } else {
        println!("{}", "This package is not installed".red().bold())
    }
}

fn read_sidecar_for_binary_name(package: String) -> String {
    let sidecar_file_path = format!(
        "{}/.winch/bin/{}.json",
        dirs::home_dir().unwrap().display(),
        package
    );

    let binary_name = match std::fs::read_to_string(sidecar_file_path) {
        Ok(content) => {
            let json: serde_json::Value = serde_json::from_str(&content).unwrap();
            json["binary_name"].as_str().unwrap().to_string()
        }
        Err(_) => {
            panic!("Failed to read sidecar file for package: {}", package);
        }
    };

    binary_name
}
