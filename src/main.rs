mod led;

use led::{Led, LedColor, LedInterface};

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
