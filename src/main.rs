use std::io::{self, Write};
use std::process::Command;

fn execute_command(command: &str) -> Result<(), std::io::Error> {
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Command '{}' failed with exit code {:?}",
                command, output.status.code()
            ),
        ))
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut input = String::new();

    print!("Enter a value for LED control: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
  
    match input.trim() {
        "on" => execute_command("echo 0 > /sys/class/leds/red-led/brightness")?,
        "off" => execute_command("echo 1 > /sys/class/leds/red-led/brightness")?,
        "heartbeat" => execute_command("echo heartbeat | tee /sys/class/leds/sys_led/trigger")?,
        _ => println!("Invalid input!"),
    }

    Ok(())
}