use std::{error::Error, io::Read};
use ssh2::{Session, Channel};

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