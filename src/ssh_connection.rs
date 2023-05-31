use std::{net::TcpStream, error::Error, io::Read};
use ssh2::{Session, Channel};

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

pub fn execute_command(session: Session, command: &str) -> Result<String, Box<dyn Error>> {
    let mut channel = new_channel(session)?;
    channel.exec(command)?;

    let mut command_output = String::new();
    channel.read_to_string(&mut command_output)?;

    close_channel(channel)?;

    Ok(command_output)
}

fn new_channel(session: Session) -> Result<Channel, Box<dyn Error>> {
    let channel = session.channel_session()?;
    Ok(channel)
}

fn close_channel(mut channel: Channel) ->Result<(), Box<dyn Error>> {
    channel.wait_close()?;
    Ok(())
}