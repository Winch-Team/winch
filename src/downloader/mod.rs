use std::{collections::HashMap, io::{Cursor, Write}, path};
use reqwest::{self, header};
use serde_json::Value;
use zip_extract;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io;
use std::process::Command;

pub(crate) fn download(package: String, version: Option<String>) {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, 
        header::HeaderValue::from_static("Mozilla/5.0...."));

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build client");
    // println!("{:?}", download_link);
    
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
    let latest = version[0].as_str().unwrap();
    
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
    
   
    let build_steps = build_steps.unwrap(); // remove Ok from Result

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
}

fn get_json(path: String) -> Result<Value, Box<dyn std::error::Error>> {
    let file_path = Path::new(&path);

    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader)?;

    Ok(json)
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