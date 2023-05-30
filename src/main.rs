use std::{error::Error, net::TcpStream, io::{Read, Write}, path::Path, fs, process::Command};
use ssh2::Session;
use dotenv;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    // let tcp = TcpStream::connect("192.168.0.121:22")?;
    // let mut sess = Session::new()?;

    // sess.set_tcp_stream(tcp);
    // sess.handshake()?;

    // match sess.userauth_password(dotenv::var("USERNAME").unwrap().as_str(), dotenv::var("PASSWORD").unwrap().as_str()) {
    //     Ok(_) => (),
    //     Err(error) => panic!("An error ocurred on trying to authenticate: {:?}", error)
    // };

    // let (mut remote_file, stat) = sess.scp_recv(Path::new("/var/www/html/interno/cron-job/notifications/negativas/log/negativas-2023-05-24.log"))?;
    // println!("Remote file stats: {}", stat.size());
    // let mut contents: Vec<u8> = Vec::new();
    // remote_file.read_to_end(&mut contents)?;

    // let mut file = fs::File::create("/home/rschuquel/remote_log.log")?;
    // file.write(&contents)?;

    // remote_file.send_eof()?;
    // remote_file.wait_eof()?;
    // remote_file.close()?;
    // remote_file.wait_close()?;

    let mut file = fs::File::open("/home/rschuquel/remote_log.log")?;
    let mut contents = String::new();
    fs::File::read_to_string(&mut file, &mut contents)?;

    let mut alerts: Vec<&str> = vec![];

    contents.lines().for_each(|line| {
        if line.contains("UnimedVTRP.ALERT") {
            alerts.push(line);
        }
    });

    println!("{:?}", alerts);

    let mut send_notify = Command::new("notify-send");
    send_notify.arg("'Um novo alerta foi detectado'");
    let send_notify_response = match send_notify.output() {
        Ok(success_output) => success_output,
        Err(error) => panic!("An error ocurred on send the notification: {:?}", error)
    };

    println!("{:?}", send_notify_response);

    Ok(())
}
