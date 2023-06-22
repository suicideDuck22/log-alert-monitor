use std::{process::Command, error::Error};
use dotenv;
use chrono::prelude::*;
use clap::Parser;

mod ssh_connection;
mod info_file;
mod remote_log;

#[derive(Debug)]
#[derive(Parser)]
struct Args {
    #[arg(short('a'), long("server_address"))]
    server_address: String,
    #[arg(short('n'), long("server_name"))]
    server_name: String,
    #[arg(short('p'), long("app_name"))]
    application_name: String,
    #[arg(short('l'), long("logs_path"))]
    logs_path: String,
}

struct AuthData {
    username: String,
    password: String
}

impl AuthData {
    fn new() -> AuthData {
        let username = dotenv::var("USERNAME").expect("Env variable `USERNAME` should be set");
        let password = dotenv::var("PASSWORD").expect("Env variable `PASSWORD` should be set");
        AuthData { username, password }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    dotenv::dotenv().ok();

    let mut args = Args::parse();
    let auth_data = AuthData::new();
    let current_date = Utc::now().format("%Y-%m-%d").to_string();
    args.logs_path = args.logs_path.replace("YYYY-MM-DD", &current_date);

    let session = ssh_connection::session::new_authenticated_session(
        &args.server_address,
        &auth_data.username,
        &auth_data.password
    ).unwrap();

    let mut info_file = match info_file::open(&args.server_address, &args.application_name) {
        Ok(file) => file,
        Err(error) => panic!("Error ocurred on trying to read or create the info file: {:?}", error)
    };

    let info_file_content = info_file::get_content(&mut info_file);

    let current_day_log_lines_count = remote_log::get_lines_quantity(
        session.clone(),
        &args.logs_path
    )?;

    let mut remote_log_alerts: String = String::new();
    if info_file_content.is_empty() == true {
        let remote_alerts = remote_log::read_all(session.clone(), &args.logs_path)?;
        remote_log_alerts.push_str(remote_alerts.as_str());
    } else {
        let (date, info_file_lines) = info_file::parse_infos(&info_file_content).unwrap();
        if date != current_date {
            let remote_alerts = remote_log::read_all(session.clone(), &args.logs_path)?;
            remote_log_alerts.push_str(remote_alerts.as_str());
        } else {
            let remote_alerts = remote_log::read_part(session, info_file_lines, &args.logs_path)?;
            remote_log_alerts.push_str(&remote_alerts);
        }
    }

    if remote_log_alerts.is_empty() == false {
        let notification_message = format!("New alert detected at {} application {}", &args.server_name, &args.application_name);
        Command::new("wsl-notify-send.exe").args(["--category", "$WSL_DISTRO_NAME", &notification_message]).spawn().unwrap();
    }

    if current_day_log_lines_count.is_empty() == true {
        return Ok(())
    }

    match info_file::insert_infos(&args.server_address, current_day_log_lines_count, &args.application_name) {
        Ok(_) => (),
        Err(error) => panic!("Error on insert new file infos: {}", error)
    };

    Ok(())
}
