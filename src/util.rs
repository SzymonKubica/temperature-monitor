use core::str::from_utf8;

use arduino_hal::{Delay, I2c};
use lcd_lcm1602_i2c::{Lcd, Backlight};
use numtoa::NumToA;

pub struct Display<'a> {
    lcd: Lcd<'a, I2c, Delay>,
}

impl<'a> Display<'a> {
    pub fn new(lcd: Lcd<'a, I2c, Delay>) -> Self {
        Self { lcd }
    }

    pub fn print_temperature(&mut self, measurement: i16) {
        self.lcd.write_str("Temperature:").unwrap();
        let mut buf = [0u8; 4];
        write_temperature(measurement, &mut buf);
        match from_utf8(&buf) {
            Ok(output_str) => self.lcd.write_str(output_str).unwrap(),
            Err(_) => self.lcd.write_str("UTF8 Error").unwrap(),
        }
    }

    pub fn print_humidity(&mut self, measurement: u16) {
        self.lcd.write_str("Humidity:  ").unwrap();
        let mut buf = [0u8; 5];
        write_humidity(measurement, &mut buf);
        match from_utf8(&buf) {
            Ok(output_str) => self.lcd.write_str(output_str).unwrap(),
            Err(_) => self.lcd.write_str("UTF8 Error").unwrap(),
        }
    }
    pub fn clear(&mut self) {
        self.lcd.clear().unwrap();
    }
    pub fn new_line(&mut self) {
        self.lcd.set_cursor(1, 0).unwrap();
    }
    pub fn backlightOn(&mut self) {
        self.lcd.backlight(Backlight::On).unwrap();
    }
    pub fn backlightOff(&mut self) {
        self.lcd.backlight(Backlight::Off).unwrap();
    }
}

fn put_char_at(c: char, buf_index: usize, out_buf: &mut [u8]) {
    c.encode_utf8(&mut out_buf[buf_index..]);
}

pub fn write_temperature(measured_temperature: i16, out_buf: &mut [u8; 4]) {
    let mut temp_buf = [0u8; 8];
    let temperature_str = measured_temperature.numtoa_str(10, &mut temp_buf);
    // The DST reports temperature in tenths of a degree, thus the expected
    // maximum length of the demperature string is 3 (e.g. 17.0 deg Celcius).
    // Note that DST will never report a number which is negative as its supported
    // measurement range is 0-50 degrees with accuracy +/- 2 degrees.
    let mut chars = temperature_str.chars();
    match temperature_str.len() {
        2 => put_char_at(' ', 0, out_buf),
        3 => put_char_at(chars.nth(0).unwrap(), 0, out_buf),
        _ => panic!(),
    };
    put_char_at(chars.nth(0).unwrap(), 1, out_buf);
    put_char_at('.', 2, out_buf);
    put_char_at(chars.nth(0).unwrap(), 3, out_buf);
}

pub fn write_humidity(measured_humidity: u16, out_buf: &mut [u8; 5]) {
    let mut temp_buf = [0u8; 8];
    let humidity_str = measured_humidity.numtoa_str(10, &mut temp_buf);
    // The same length constraint is true for the humidity reading which is reported
    // in tenths of a percent and so the fange of possible values is 0-999
    let mut chars = humidity_str.chars();
    match humidity_str.len() {
        2 => put_char_at(' ', 0, out_buf),
        3 => put_char_at(chars.nth(0).unwrap(), 0, out_buf),
        _ => panic!(),
    };
    put_char_at(chars.nth(0).unwrap(), 1, out_buf);
    put_char_at('.', 2, out_buf);
    put_char_at(chars.nth(0).unwrap(), 3, out_buf);
    put_char_at('%', 4, out_buf);
}
