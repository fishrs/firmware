use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::thread::sleep;
use std::time::Duration;
use image::{ImageBuffer, Rgb};

const LCD_WIDTH: u16 = 240;
const LCD_HEIGHT: u16 = 320;

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

    // Initialize the LCD (Example for ILI9341 controller)
    lcd_command(&mut spi, &mut dc_pin, 0x01); // Software reset
    sleep(Duration::from_millis(5));
    lcd_command(&mut spi, &mut dc_pin, 0x28); // Display off
    lcd_command(&mut spi, &mut dc_pin, 0xCF);
    lcd_data(&mut spi, &mut dc_pin, 0x00);
    lcd_data(&mut spi, &mut dc_pin, 0x83);
    lcd_data(&mut spi, &mut dc_pin, 0x30);
    // Add more initialization commands as required by your LCD controller
    lcd_command(&mut spi, &mut dc_pin, 0x29); // Display on

    // Turn on the backlight
    bl_pin.set_high();

    // Load the image
    let img = image::open("test.jpg").expect("Failed to open image");

    // Resize the image to fit the LCD screen
    let img = img.resize(LCD_WIDTH as u32, LCD_HEIGHT as u32, image::imageops::FilterType::Nearest);

    // Send the image to the LCD
    send_image_to_lcd(&mut spi, &mut dc_pin, &img.to_rgb8());
}

fn lcd_command(spi: &mut Spi, dc_pin: &mut OutputPin, cmd: u8) {
    dc_pin.set_low();
    spi.write(&[cmd]).expect("Failed to send command");
}

fn lcd_data(spi: &mut Spi, dc_pin: &mut OutputPin, data: u8) {
    dc_pin.set_high();
    spi.write(&[data]).expect("Failed to send data");
}

fn send_image_to_lcd(spi: &mut Spi, dc_pin: &mut OutputPin, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
    lcd_command(spi, dc_pin, 0x2C); // Memory write

    for y in 0..LCD_HEIGHT {
        for x in 0..LCD_WIDTH {
            let pixel = img.get_pixel(x as u32, y as u32);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            // ILI9341 expects 16-bit color (5-6-5 format)
            let color: u16 = ((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | (b as u16 >> 3);
            lcd_data(spi, dc_pin, (color >> 8) as u8); // High byte
            lcd_data(spi, dc_pin, (color & 0xFF) as u8); // Low byte
        }
    }
}
