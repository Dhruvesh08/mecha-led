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

    print!("Enter an LED name (red/green/blue): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    let led_name = match input.trim() {
        "red" => "/sys/class/leds/red-led/brightness",
        "green" => "/sys/class/leds/green-led/brightness",
        "blue" => "/sys/class/leds/blue-led/brightness",
        _ => {
            println!("Invalid LED name!");
            return Ok(());
        }
    };

    print!("Enter an LED state (on/off): ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "on" => execute_command(&format!("echo 1 > {}", led_name))?,
        "off" => execute_command(&format!("echo 0 > {}", led_name))?,
        _ => println!("Invalid LED state!"),
    }

    Ok(())
}
