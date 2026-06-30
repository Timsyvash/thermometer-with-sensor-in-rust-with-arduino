#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut trig = pins.d11.into_output();
    let echo = pins.d10.into_floating_input();

    let mut red_led = pins.d2.into_output();
    let mut green_led = pins.d3.into_output();
    let mut blue_led = pins.d5.into_output();

    red_led.set_high();
    green_led.set_high();
    blue_led.set_high();

    ufmt::uwriteln!(&mut serial, "Proximity RGB Indicator Started!\r").unwrap();

    loop {
        trig.set_low();
        arduino_hal::delay_us(5);
        trig.set_high();
        arduino_hal::delay_us(10);
        trig.set_low();

        let mut timeout = 0;
        while echo.is_low() {
            timeout += 1;
            arduino_hal::delay_us(1);
            if timeout > 10000 { break; }
        }

        let mut microseconds: u32 = 0;
        while echo.is_high() {
            arduino_hal::delay_us(1);
            microseconds += 1;
            if microseconds > 23000 { break; }
        }

        let distance_cm = (microseconds * 34) / 2000;

        ufmt::uwriteln!(&mut serial, "Distance: {} cm\r", distance_cm).unwrap();

        if distance_cm > 0 && distance_cm <= 15 {
            red_led.set_low();
            green_led.set_high();
            blue_led.set_high();
        } else if distance_cm > 15 && distance_cm <= 60 {
            red_led.set_low();
            green_led.set_low();
            blue_led.set_high();
        } else {
            red_led.set_high();
            green_led.set_high();
            blue_led.set_low();
        }

        arduino_hal::delay_ms(100);
    }
}