use std::{net::TcpStream, error::Error};
use ssh2::Session;

pub fn new_authenticated_session(server_address: &str, username: &str, password: &str) -> Result<Session, Box<dyn Error>> {
    let port = ":22";
    let server_addr_with_port = format!("{}{}", server_address, port);
    let server_tcp = TcpStream::connect(server_addr_with_port)?;
    let mut session = Session::new()?;

    session.set_tcp_stream(server_tcp);
    session.handshake()?;
    session.userauth_password(username, password)?;

    Ok(session)
}