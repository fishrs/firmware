use rppal::gpio::{Gpio, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::thread::sleep;
use std::time::Duration;

const LCD_WIDTH: u16 = 240;
const LCD_HEIGHT: u16 = 320;

const LCD_CMD: u8 = 0;
const LCD_DATA: u8 = 1;

fn main() {
    // Setup GPIO pins
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let mut dc_pin = gpio.get(24).expect("Failed to get GPIO 24").into_output();
    let mut rst_pin = gpio.get(25).expect("Failed to get GPIO 25").into_output();
    let mut bl_pin = gpio.get(18).expect("Failed to get GPIO 18").into_output();

    // Setup SPI
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 32_000_000, Mode::Mode0)
        .expect("Failed to initialize SPI");

    // Reset the LCD
    rst_pin.set_low();
    sleep(Duration::from_millis(100));
    rst_pin.set_high();
    sleep(Duration::from_millis(100));

    // Initialize the LCD
    // This depends on the specific LCD controller you have
    // Here is an example for an ILI9341 controller
    lcd_command(&mut spi, &mut dc_pin, 0x01); // Software reset
    sleep(Duration::from_millis(5));
    lcd_command(&mut spi, &mut dc_pin, 0x28); // Display off
    // Add more initialization commands as required by your LCD controller

    // Turn on the backlight
    bl_pin.set_high();
}

fn lcd_command(spi: &mut Spi, dc_pin: &mut OutputPin, cmd: u8) {
    dc_pin.set_low();
    spi.write(&[cmd]).expect("Failed to send command");
}

fn lcd_data(spi: &mut Spi, dc_pin: &mut OutputPin, data: u8) {
    dc_pin.set_high();
    spi.write(&[data]).expect("Failed to send data");
}
