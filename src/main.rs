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
    let res = led_red.with_exported(|| {
        let period_ms = 100;
        let duration_ms = 999999999;
        led_red.set_direction(Direction::Low)?;
        let iterations = duration_ms / period_ms / 2;
        for _ in 0..iterations {
            led_red.set_value(0)?;
            sleep(Duration::from_millis(period_ms));
            led_red.set_value(1)?;
            sleep(Duration::from_millis(period_ms));
        }
        led_red.set_value(0)?;
        Ok(())
});
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
