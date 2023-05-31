use std::{net::TcpStream, error::Error, io::Read};
use ssh2::{Session, Channel};

pub fn new_session(server_address: &str) -> Result<Session, Box<dyn Error>> {
    let port = ":22";
    let server_addr_with_port = format!("{}{}", server_address, port);
    let server_tcp = TcpStream::connect(server_addr_with_port)?;
    let mut session = Session::new()?;

    session.set_tcp_stream(server_tcp);
    session.handshake()?;
    Ok(session)
}

pub fn authenticate_session(session: Session, username: &str, password: &str) -> Result<Session, Box<dyn Error>> {
    session.userauth_password(username, password)?;
    Ok(session)
}

pub fn new_channel(authenticated_session: Session) -> Result<Channel, Box<dyn Error>> {
    let channel = authenticated_session.channel_session()?;
    Ok(channel)
}

pub fn execute_command(channel: &mut Channel, command: &str) -> Result<String, Box<dyn Error>> {
    channel.exec(command)?;

    let mut command_output = String::new();
    channel.read_to_string(&mut command_output)?;

    Ok(command_output)
}