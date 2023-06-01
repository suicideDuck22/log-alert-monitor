use std::{process::Command};
use dotenv;
use chrono::prelude::*;
use clap::Parser;

mod ssh_connection;
mod info_file;
mod remote_log;

#[derive(Debug)]
#[derive(Parser)]
struct Args {
    #[arg()]
    server_address: String,
    server_name: String
}

fn main() {
    dotenv::dotenv().ok();

    let args = Args::parse();

    println!("{:#?}", args);

    let current_date = Utc::now().format("%Y-%m-%d").to_string();
    let server_name = "srv-www-prod01";
    let server_address = "192.168.0.121";
    let username = dotenv::var("USERNAME").expect("Env variable `USERNAME` should be set");
    let password = dotenv::var("PASSWORD").expect("Env variable `PASSWORD` should be set");
    let session = ssh_connection::new_authenticated_session(
        server_address,
        username.as_str(),
        password.as_str()
    ).unwrap();

    let mut info_file = match info_file::open(server_address) {
        Ok(file) => file,
        Err(error) => panic!("Error ocurred on trying to read or create the info file: {:?}", error)
    };

    let info_file_content = info_file::get_content(&mut info_file);
    let current_day_log_lines_count = match remote_log::get_lines_quantity(
        session.clone()
    ) {
        Ok(quantity) => quantity,
        Err(error) => panic!("Error on executing the command or reading the output: {:#?}", error)
    };

    let mut alerts: String = String::new();
    if info_file_content.is_empty() == true {
        match remote_log::read_all_searching_alerts(session.clone()) {
            Ok(remote_alerts) => alerts.push_str(remote_alerts.as_str()),
            Err(error) => panic!("Error on executing the command or reading the output: {:#?}", error)
        };
    } else {
        let (date, info_file_lines) = info_file::parse_infos(&info_file_content).unwrap();
        if date != current_date {
            match remote_log::read_all_searching_alerts(session.clone()) {
                Ok(remote_alerts) => alerts.push_str(remote_alerts.as_str()),
                Err(error) => panic!("Error on executing the command or reading the output: {:#?}", error)
            };
        } else {
            match remote_log::read_since_searching_alerts(session, info_file_lines) {
                Ok(remote_alerts) => alerts.push_str(&remote_alerts),
                Err(error) => panic!("Error on executing the command or reading the output: {:#?}", error)
            }
        }
    }

    println!("Logs: {}", alerts);

    if alerts.is_empty() == false {
        let notification_message = format!("New alert detected at {}", server_name);
        Command::new("wsl-notify-send.exe").args(["--category", "$WSL_DISTRO_NAME", &notification_message]).spawn().unwrap();
    }

    match info_file::insert_infos(server_address, current_day_log_lines_count) {
        Ok(_) => (),
        Err(error) => panic!("Error on insert new file infos: {}", error)
    };
}
