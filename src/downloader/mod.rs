use std::{collections::HashMap, io::{Cursor, Write}, path};
use reqwest::{self, header};
use serde_json::Value;
use zip_extract;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io;
use std::process::Command;
use std::os::unix::fs::PermissionsExt;
use dirs;
use std::path::PathBuf;
use std::fs;

#[allow(unused_variables)]

pub(crate) fn download(package: String, version: Option<String>) {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, 
        header::HeaderValue::from_static("Mozilla/5.0...."));

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build client");
    
    let package_info_resp = client.get(format!("https://api.winchteam.dev/getInfo/{}", package))
        .send()
        .unwrap()
        .text();
    let json_parse: Value = match serde_json::from_str(std::str::from_utf8(&package_info_resp.unwrap().trim().as_bytes()).unwrap()) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Failed to parse JSON: {}", error);
            return;
        }
    };
    let author = json_parse["author"].as_str().unwrap();
    let version = json_parse["versions"].as_array().unwrap().iter().filter(|x| {
        return !x.is_null();
    }).collect::<Vec<_>>();
    let latest = version[version.len() - 1].as_str().unwrap();
    
    let download_link = format!("https://api.winchteam.dev/download/{}/{}/{}", package, author, latest);

    let resp = reqwest::blocking::get(download_link)
        .unwrap()
        .json::<HashMap<String, String>>();

    let respbinding = resp.unwrap();
    let download_link = respbinding.get("download_url");

    let github_resp = client.get(download_link.unwrap().to_string())
        .send()
        .unwrap()
        .text();

    let object: Value = serde_json::from_str(std::str::from_utf8(&github_resp.unwrap().as_bytes()).unwrap()).unwrap();
    let zip_url = object["zipball_url"].as_str().unwrap();

    let zip_resp = client.get(zip_url)
        .send()
        .unwrap()
        .bytes();
    let archive: Vec<u8> = zip_resp.unwrap().to_vec();
    let target_dir = std::env::current_dir().unwrap().join(path::Path::new("temp"));
    zip_extract::extract(Cursor::new(archive), &target_dir, true).expect("Failed to extract zip");

    let build_steps = get_build_steps_from_json("./temp/.winch/install.json".to_string());
    
   
    let build_steps = build_steps.unwrap();

    // iterate over the build steps and execute them after asking for user confirmation
    
    for step in build_steps.iter() {
        println!("Executing step: {}", step);
        print!("Do you want to continue? (y/n) ");
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
    
        if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "Y"{
            let (shell, shell_arg) = if cfg!(target_os = "windows") {
                ("powershell", "-Command") // PowerShell invocation
            } else {
                ("sh", "-c") // Unix-like shells
            };
            //TODO: Nushell support? I believe it defaults to powershell or bash
            let output = Command::new(shell)
                .arg(shell_arg)
                .arg(step)
                .output()
                .expect("Failed to execute command");
    
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        } else {
            println!("aborting install!");
            break;
        }
    }

    let home_dir = dirs::home_dir();
    let target_dir = home_dir.expect("No home dir found").join(".winch/bin");

    fs::create_dir_all(&target_dir);

    let mut temp_dir = PathBuf::from(Path::new("./temp"));
    let exec_dir = get_executable_dir_from_json("./temp/.winch/install.json".to_string()).unwrap();
    temp_dir.push(exec_dir);
    let exec_path = temp_dir;

    let executables = find_executables(&exec_path);
    for executable in executables {
        let file_name = executable.file_name();
        let target_path = if let Some(name) = file_name {
            target_dir.join(name)
        } else {
            panic!("Unable to use target_path");
        };
        fs::rename(&executable, &target_path);
    }

    fs::remove_dir_all("./temp");
}

fn get_build_steps_from_json(path: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file_path = Path::new(&path);

    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader)?;

    // Extract the install steps
    let steps = json["build_steps"]
        .as_array()
        .ok_or("build_steps is not an array")?
        .iter()
        .map(|step| step.as_str().ok_or("step is not a string").map(|s| s.to_string()))
        .collect::<Result<Vec<String>, _>>()?;

    Ok(steps)
}

fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let permissions = metadata.permissions();
        permissions.mode() & 0o111 != 0
    } else {
        false
    }
}

fn find_executables(directory: &Path) -> Vec<PathBuf> {
    let mut executables = Vec::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && is_executable(&path) {
                    executables.push(path);
                }
            }
        }
    }
    executables
}

fn get_executable_dir_from_json(path: String) -> Result<String,Box<dyn std::error::Error>>{
    let file_path = Path::new(&path);

    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader)?;
    
    let execdir = json["executable_dir"].as_str();
    Ok(execdir.expect("Executable dir not found in json").to_string())
} 
