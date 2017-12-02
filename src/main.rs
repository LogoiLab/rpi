extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::time::Duration;
use std::thread::sleep;
use std::env;

struct Arguments {
    pin_red: u64,
    pin_green: u64,
    pin_blue: u64,
}

// Export a GPIO for use.  This will not fail if already exported
fn network_status(error_num: u64, args: &Arguments) {
    let led_red = Pin::new(args.pin_red);
    let led_green = Pin::new(args.pin_green);
    let led_blue = Pin::new(args.pin_blue);
    led_red.with_exported(|| {
        led_red.set_direction(Direction::Low)?;
        led_red.set_value(0)?;
        Ok(())
    });
    led_green.with_exported(|| {
        led_green.set_direction(Direction::Low)?;
        led_green.set_value(0)?;
        Ok(())
    });
    led_blue.with_exported(|| {
        led_blue.set_direction(Direction::Low)?;
        led_blue.set_value(0)?;
        Ok(())
    });
    let res: Result<(),sysfs_gpio::Error> = match error_num {
        0 => {led_green.with_exported(|| {
                led_green.set_direction(Direction::Low)?;
                led_green.set_value(1)?;
                Ok(())
            });Ok(())},
        1 => {led_red.with_exported(|| {
                led_red.set_direction(Direction::Low)?;
                led_red.set_value(1)?;
                Ok(())
            });Ok(())},
        3 => {led_blue.with_exported(|| {
                led_blue.set_direction(Direction::Low)?;
                led_blue.set_value(1)?;
                Ok(())
            });Ok(())},
        _ => {Ok(())},
    };
}

fn main() {
    let args: Arguments = Arguments {
        pin_red: 4,
        pin_green: 3,
        pin_blue: 14,
    };
    network_status(0, &args);
    sleep(Duration::from_millis(1000));
    network_status(1, &args);
    sleep(Duration::from_millis(1000));
    network_status(2, &args);
    sleep(Duration::from_millis(1000));
    network_status(3, &args);
}
