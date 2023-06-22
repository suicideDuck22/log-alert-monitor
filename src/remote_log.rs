use std::error::Error;
use crate::ssh_connection;
use ssh2::Session;

pub fn get_lines_quantity(session: Session, log_full_path: &str) -> Result<String, Box<dyn Error>> {
    let command = format!("wc -l < {}", log_full_path);
    let lines_quantity = ssh_connection::channel::execute_command(session, command.as_str())?;
    Ok(lines_quantity)
}

pub fn read_all(session: Session, log_full_path: &str) -> Result<String, Box<dyn Error>> {
    let command = format!("cat {} | grep ALERT", log_full_path);
    let alerts = ssh_connection::channel::execute_command(session, command.as_str())?;
    Ok(alerts)
}

pub fn read_part(session: Session, past_line_quantity: &str, log_full_path: &str) -> Result<String, Box<dyn Error>> {
    let current_lines_quantity = get_lines_quantity(session.clone(), &log_full_path)?;
    let read_since = current_lines_quantity.trim().parse::<u32>()? - past_line_quantity.parse::<u32>()?;
    let command = format!("tail -n {} {} | grep ALERT", read_since, &log_full_path);
    let alerts = ssh_connection::channel::execute_command(session, command.as_str())?;
    Ok(alerts)
}