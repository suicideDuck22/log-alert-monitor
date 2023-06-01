use std::error::Error;
use crate::ssh_connection;
use ssh2::Session;

pub fn get_lines_quantity(session: Session) -> Result<String, Box<dyn Error>> {
    let command = "wc -l < /var/www/html/interno/cron-job/notifications/negativas/log/negativas-2023-06-01.log";
    let lines_quantity = ssh_connection::execute_command(session, command)?;
    Ok(lines_quantity)
}

pub fn read_all_searching_alerts(session: Session) -> Result<String, Box<dyn Error>> {
    let command = "cat /var/www/html/interno/cron-job/notifications/negativas/log/negativas-2023-06-01.log";
    let alerts = ssh_connection::execute_command(session, command)?;
    Ok(alerts)
}

pub fn read_since_searching_alerts(session: Session, past_line_quantity: &str) -> Result<String, Box<dyn Error>> {
    let current_lines_quantity = get_lines_quantity(session.clone())?;
    let read_since = current_lines_quantity.trim().parse::<u32>()? - past_line_quantity.parse::<u32>()?;
    let command = format!("tail -n {} /var/www/html/interno/cron-job/notifications/negativas/log/negativas-2023-06-01.log", read_since);
    let alerts = ssh_connection::execute_command(session, command.as_str())?;
    Ok(alerts)
}