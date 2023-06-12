use std::{fs, error::Error, io::Read};
use chrono::prelude::*;

pub fn open(server_address: &str, application_name: &str) -> Result<fs::File, Box<dyn Error>> {
    let path = format!("./remote-servers-infos/{}-{}.info", server_address, application_name);
    let file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(_) => match fs::File::create(&path) {
            Ok(_) => fs::File::open(&path).unwrap(),
            Err(error) => panic!("Error: {:?}", error)
        }
    };

    Ok(file)
}

pub fn get_content(file: &mut fs::File) -> String {
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    file_content
}

pub fn insert_infos(server_address: &str, log_line_quantity: String, application_name: &str) -> Result<(), Box<dyn Error>> {
    let path = format!("./remote-servers-infos/{}-{}.info", server_address, application_name);
    let current_date = Utc::now().format("%Y-%m-%d").to_string();
    let new_content = format!("{}|{}", current_date, log_line_quantity);
    fs::write(path, new_content.as_bytes()).unwrap();
    Ok(())
}

pub fn parse_infos(content: &String) -> Result<(&str, &str), Box<dyn Error>> {
    let infos: Vec<&str> = content.trim().split('|').collect();
    let date  = infos.get(0).unwrap();
    let lines = infos.get(1).unwrap();
    Ok((date, lines))
}