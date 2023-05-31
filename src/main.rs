use std::{error::Error, io::{Read, Write}, path::Path, fs, process::Command, sync::mpsc::channel};
use dotenv;
use chrono::prelude::*;

mod ssh_connection;

fn main() {
    dotenv::dotenv().ok();

    let current_date = Utc::now().format("%Y-%m-%d").to_string();

    let new_session = match ssh_connection::new_session("192.168.0.121"){
        Ok(session) => session,
        Err(error) => panic!("An error ocurred on trying to create a new session: {:?}", error)
    };

    let username = dotenv::var("USERNAME").expect("Env variable `USERNAME` should be set");
    let password = dotenv::var("PASSWORD").expect("Env variable `PASSWORD` should be set");
    let authenticated_session = match ssh_connection::authenticate_session(new_session, username.as_str(), password.as_str()) {
        Ok(auth_session) => auth_session,
        Err(error) => panic!("An error ocurred on trying to authenticate: {:?}", error)
    };

    let mut channel = match ssh_connection::new_channel(authenticated_session) {
        Ok(channel) => channel,
        Err(error) => panic!("Error on creating the channel session: {:?}", error)
    };

    let mut file = match fs::File::open("./log-informations.txt") {
        Ok(file) => file,
        Err(_) => match fs::File::create("./log-informations.txt") {
            Ok(file) => file,
            Err(error) => panic!("Error: {:?}", error)
        }
    };

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let current_day_log_lines_count = match ssh_connection::execute_command(
        &mut channel,
        "wc -l < /var/www/html/interno/cron-job/notifications/negativas/log/negativas-2023-05-31.log"
    ) {
        Ok(count) => count,
        Err(error) => panic!("Error on executing the command or reading the output: {:#?}", error)
    };

    if file_content.is_empty() {
        
    }

    println!("Alerts: {}", current_day_log_lines_count);
    channel.wait_close().unwrap();

    // let (mut remote_file, stat) = sess.scp_recv(Path::new(dotenv::var("LOG_PATH").unwrap().as_str()))?;
    // println!("Remote file stats: {}", stat.size());
    // let mut contents: Vec<u8> = Vec::new();
    // remote_file.read_to_end(&mut contents)?;

    // let mut file = fs::File::create("/home/rschuquel/remote_log.log")?;
    // file.write(&contents)?;

    // remote_file.send_eof()?;
    // remote_file.wait_eof()?;
    // remote_file.close()?;
    // remote_file.wait_close()?;

    // let mut file = fs::File::open("/home/rschuquel/remote_log.log")?;
    // let mut contents = String::new();
    // fs::File::read_to_string(&mut file, &mut contents)?;

    // let mut alerts: Vec<&str> = vec![];

    // contents.lines().for_each(|line| {
    //     if line.contains("UnimedVTRP.ALERT") {
    //         alerts.push(line);
    //     }
    // });

    // println!("{:?}", alerts);

    // let send_notify = Command::new("zsh").arg("-c").arg("wsl-notify-send.exe --category $WSL_DISTRO_NAME 'Hello'").spawn();
    // match send_notify {
    //     Ok(_) => (),
    //     Err(error) => panic!("An error ocurred on send the notification: {:?}", error)
    // };
}
