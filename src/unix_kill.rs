use std::io;
use std::process::Command;

pub fn signal_by_name(pid: u32, name: &str) -> io::Result<()> {
    Command::new("kill")
        .args(["-s", name])
        .arg(pid.to_string())
        .spawn()?
        .wait()?;
    Ok(())
}

pub fn signal_interrupt(pid: u32) -> io::Result<()> {
    signal_by_name(pid, "SIGINT")
}