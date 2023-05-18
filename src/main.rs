#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern crate arduino_hal;
extern crate dht11;
extern crate lcd_lcm1602_i2c;
extern crate numtoa;

use dht11::Dht11;
use lcd_lcm1602_i2c::Lcd;

use crate::{button::Button, util::Display};

mod button;
mod util;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    loop {
        let peripherals = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(peripherals);

        let pin = pins.a3.into_opendrain_high();

        const LCD_ADDRESS: u8 = 0x27; // Address depends on hardware, see link below

        // Create a I2C instance, needs to implement embedded_hal::blocking::i2c::Write, this
        // particular uses the arduino_hal crate for avr microcontrollers like the arduinos.
        let mut i2c = arduino_hal::I2c::new(
            peripherals.TWI,              //
            pins.a4.into_pull_up_input(), // use respective pins
            pins.a5.into_pull_up_input(),
            200000,
        );
        let mut delay = arduino_hal::Delay::new();
        let mut delay2 = arduino_hal::Delay::new();

        let button_pin = pins.a1.into_floating_input();
        let mut button = Button::new(button_pin);

        let mut display = Display::new(
            Lcd::new(&mut i2c, &mut delay)
                .address(LCD_ADDRESS)
                .cursor_on(false) // no visible cursos
                .rows(2) // two rows
                .init()
                .unwrap(),
        );

        let mut dht11 = Dht11::new(pin);

        let mut temperature: i16 = 0;
        let mut humidity: u16 = 0;

        // Wait so that the DHT sensor has some time to start.
        arduino_hal::delay_ms(1000);
        display.clear();
        display.write_str("Test");
        loop {
            let measurement_result = dht11.perform_measurement(&mut delay2);
            if let Ok(measurement) = measurement_result {
                let new_temperature = measurement.temperature;
                let new_humidity = measurement.humidity;
                if new_temperature != temperature || new_humidity != humidity {
                    temperature = new_temperature;
                    humidity = new_humidity;
                    display.clear();
                    display.print_temperature(temperature);
                    display.new_line();
                    display.print_humidity(humidity);
                }
            }
            for _ in 1..100 {
                if button.toggle_detected() {
                    display.toggle_backlight();
                }
                arduino_hal::delay_ms(50);
            }
        }
    }
}
