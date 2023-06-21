use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

pub enum LedColor {
    Red,
    Green,
    Blue,
}

pub trait LedInterface {
    fn set_device(&mut self, color: LedColor, value: u8) -> Result<(), io::Error>;
    fn get_device(&self, color: LedColor) -> Result<i32, io::Error>;
    fn info(&self) -> Result<(i32, i32, i32), io::Error>;
    fn get_data(&self, color: LedColor) -> Result<i32, io::Error>;
}

pub struct Led {
    pub red: String,
    pub green: String,
    pub blue: String,
}

impl LedInterface for Led {
    fn set_device(&mut self, color: LedColor, value: u8) -> Result<(), io::Error> {
        let value_str = match value {
            0 => "0",
            1 => "1",
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid value")),
        };

        let path = match color {
            LedColor::Red => &self.red,
            LedColor::Green => &self.green,
            LedColor::Blue => &self.blue,
        };

        fs::write(path, value_str)?;
        Ok(())
    }

    fn get_device(&self, color: LedColor) -> Result<i32, io::Error> {
        let path = match color {
            LedColor::Red => &self.red,
            LedColor::Green => &self.green,
            LedColor::Blue => &self.blue,
        };

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let line = reader.lines().next();

        match line {
            Some(Ok(value)) => {
                if let Ok(int_value) = value.parse::<i32>() {
                    Ok(int_value)
                } else {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid LED value"))
                }
            }
            Some(Err(err)) => Err(err),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "LED value not found")),
        }
    }

    fn info(&self) -> Result<(i32, i32, i32), io::Error> {
        let red_value = self.get_device(LedColor::Red)?;
        let green_value = self.get_device(LedColor::Green)?;
        let blue_value = self.get_device(LedColor::Blue)?;

        Ok((red_value, green_value, blue_value))
    }

    fn get_data(&self, color: LedColor) -> Result<i32, io::Error> {
        let path = match color {
            LedColor::Red => &self.red,
            LedColor::Green => &self.green,
            LedColor::Blue => &self.blue,
        };

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let line = reader.lines().next();

        match line {
            Some(Ok(value)) => {
                if let Ok(int_value) = value.parse::<i32>() {
                    Ok(int_value)
                } else {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid LED value"))
                }
            }
            Some(Err(err)) => Err(err),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "LED value not found")),
        }
    }
}

fn main() {
    let mut led = Led {
        red: "/sys/class/leds/red-led/brightness".to_string(),
        green: "/sys/class/leds/green-led/brightness".to_string(),
        blue: "/sys/class/leds/blue-led/brightness".to_string(),
    };

    match led.set_device(LedColor::Red, 1) {
        Ok(()) => println!("Red LED set to 1"),
        Err(err) => println!("Error setting Red LED: {}", err),
    }

    match led.set_device(LedColor::Green, 0) {
        Ok(()) => println!("Green LED set to 0"),
        Err(err) => println!("Error setting Green LED: {}", err),
    }

    match led.set_device(LedColor::Blue, 1) {
        Ok(()) => println!("Blue LED set to 1"),
        Err(err) => println!("Error setting Blue LED: {}", err),
    }

    match led.info() {
        Ok((red_value, green_value, blue_value)) => {
            println!("Red LED value: {}", red_value);
            println!("Green LED value: {}", green_value);
            println!("Blue LED value: {}", blue_value);
        }
        Err(err) => println!("Error getting LED info: {}", err),
    }

    match led.get_data(LedColor::Red) {
        Ok(value) => println!("Red LED data: {}", value),
        Err(err) => println!("Error getting Red LED data: {}", err),
    }
}
