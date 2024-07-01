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

    lcd_command(&mut spi, &mut dc_pin, 0xEF);
    lcd_data(&mut spi, &mut dc_pin, 0x03);
    lcd_data(&mut spi, &mut dc_pin, 0x80);
    lcd_data(&mut spi, &mut dc_pin, 0x02);

    lcd_command(&mut spi, &mut dc_pin, 0xCF);
    lcd_data(&mut spi, &mut dc_pin, 0x00);
    lcd_data(&mut spi, &mut dc_pin, 0xC1);
    lcd_data(&mut spi, &mut dc_pin, 0x30);

    lcd_command(&mut spi, &mut dc_pin, 0xED);
    lcd_data(&mut spi, &mut dc_pin, 0x64);
    lcd_data(&mut spi, &mut dc_pin, 0x03);
    lcd_data(&mut spi, &mut dc_pin, 0x12);
    lcd_data(&mut spi, &mut dc_pin, 0x81);

    lcd_command(&mut spi, &mut dc_pin, 0xE8);
    lcd_data(&mut spi, &mut dc_pin, 0x85);
    lcd_data(&mut spi, &mut dc_pin, 0x00);
    lcd_data(&mut spi, &mut dc_pin, 0x78);

    lcd_command(&mut spi, &mut dc_pin, 0xCB);
    lcd_data(&mut spi, &mut dc_pin, 0x39);
    lcd_data(&mut spi, &mut dc_pin, 0x2C);
    lcd_data(&mut spi, &mut dc_pin, 0x00);
    lcd_data(&mut spi, &mut dc_pin, 0x34);
    lcd_data(&mut spi, &mut dc_pin, 0x02);

    lcd_command(&mut spi, &mut dc_pin, 0xF7);
    lcd_data(&mut spi, &mut dc_pin, 0x20);

    lcd_command(&mut spi, &mut dc_pin, 0xEA);
    lcd_data(&mut spi, &mut dc_pin, 0x00);
    lcd_data(&mut spi, &mut dc_pin, 0x00);

    lcd_command(&mut spi, &mut dc_pin, 0xC0); // Power control 1
    lcd_data(&mut spi, &mut dc_pin, 0x23);

    lcd_command(&mut spi, &mut dc_pin, 0xC1); // Power control 2
    lcd_data(&mut spi, &mut dc_pin, 0x10);

    lcd_command(&mut spi, &mut dc_pin, 0xC5); // VCOM control 1
    lcd_data(&mut spi, &mut dc_pin, 0x3e);
    lcd_data(&mut spi, &mut dc_pin, 0x28);

    lcd_command(&mut spi, &mut dc_pin, 0xC7); // VCOM control 2
    lcd_data(&mut spi, &mut dc_pin, 0x86);

    lcd_command(&mut spi, &mut dc_pin, 0x36); // Memory Access Control
    lcd_data(&mut spi, &mut dc_pin, 0x48);

    lcd_command(&mut spi, &mut dc_pin, 0x3A); // Pixel Format Set
    lcd_data(&mut spi, &mut dc_pin, 0x55);    // 16-bit/pixel

    lcd_command(&mut spi, &mut dc_pin, 0xB1); // Frame Rate Control
    lcd_data(&mut spi, &mut dc_pin, 0x00);
    lcd_data(&mut spi, &mut dc_pin, 0x18);

    lcd_command(&mut spi, &mut dc_pin, 0xB6); // Display Function Control
    lcd_data(&mut spi, &mut dc_pin, 0x08);
    lcd_data(&mut spi, &mut dc_pin, 0x82);
    lcd_data(&mut spi, &mut dc_pin, 0x27);

    lcd_command(&mut spi, &mut dc_pin, 0xF2); // 3Gamma Function Disable
    lcd_data(&mut spi, &mut dc_pin, 0x00);

    lcd_command(&mut spi, &mut dc_pin, 0x26); // Gamma curve selected
    lcd_data(&mut spi, &mut dc_pin, 0x01);

    lcd_command(&mut spi, &mut dc_pin, 0xE0); // Set Gamma
    lcd_data(&mut spi, &mut dc_pin, 0x0F);
    lcd_data(&mut spi, &mut dc_pin, 0x31);
    lcd_data(&mut spi, &mut dc_pin, 0x2B);
    lcd_data(&mut spi, &mut dc_pin, 0x0C);
    lcd_data(&mut spi, &mut dc_pin, 0x0E);
    lcd_data(&mut spi, &mut dc_pin, 0x08);
    lcd_data(&mut spi, &mut dc_pin, 0x4E);
    lcd_data(&mut spi, &mut dc_pin, 0xF1);
    lcd_data(&mut spi, &mut dc_pin, 0x37);
    lcd_data(&mut spi, &mut dc_pin, 0x07);
    lcd_data(&mut spi, &mut dc_pin, 0x10);
    lcd_data(&mut spi, &mut dc_pin, 0x03);
    lcd_data(&mut spi, &mut dc_pin, 0x0E);
    lcd_data(&mut spi, &mut dc_pin, 0x09);
    lcd_data(&mut spi, &mut dc_pin, 0x00);

    lcd_command(&mut spi, &mut dc_pin, 0xE1); // Set Gamma
    lcd_data(&mut spi, &mut dc_pin, 0x00);
    lcd_data(&mut spi, &mut dc_pin, 0x0E);
    lcd_data(&mut spi, &mut dc_pin, 0x14);
    lcd_data(&mut spi, &mut dc_pin, 0x03);
    lcd_data(&mut spi, &mut dc_pin, 0x11);
    lcd_data(&mut spi, &mut dc_pin, 0x07);
    lcd_data(&mut spi, &mut dc_pin, 0x31);
    lcd_data(&mut spi, &mut dc_pin, 0xC1);
    lcd_data(&mut spi, &mut dc_pin, 0x48);
    lcd_data(&mut spi, &mut dc_pin, 0x08);
    lcd_data(&mut spi, &mut dc_pin, 0x0F);
    lcd_data(&mut spi, &mut dc_pin, 0x0C);
    lcd_data(&mut spi, &mut dc_pin, 0x31);
    lcd_data(&mut spi, &mut dc_pin, 0x36);
    lcd_data(&mut spi, &mut dc_pin, 0x0F);

    lcd_command(&mut spi, &mut dc_pin, 0x11); // Exit Sleep
    sleep(Duration::from_millis(120));
    lcd_command(&mut spi, &mut dc_pin, 0x29); // Display on

    // Initialize the LCD (Example for ILI9341 controller)
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
    // Set column address
    lcd_command(spi, dc_pin, 0x2A);
    lcd_data(spi, dc_pin, 0x00);
    lcd_data(spi, dc_pin, 0x00);
    lcd_data(spi, dc_pin, (LCD_WIDTH - 1 >> 8) as u8);
    lcd_data(spi, dc_pin, (LCD_WIDTH - 1 & 0xFF) as u8);

    // Set row address
    lcd_command(spi, dc_pin, 0x2B);
    lcd_data(spi, dc_pin, 0x00);
    lcd_data(spi, dc_pin, 0x00);
    lcd_data(spi, dc_pin, (LCD_HEIGHT - 1 >> 8) as u8);
    lcd_data(spi, dc_pin, (LCD_HEIGHT - 1 & 0xFF) as u8);

    // Write to RAM
    lcd_command(spi, dc_pin, 0x2C);

    // Send image data
    for pixel in img.pixels() {
        let r = pixel[0] >> 3;
        let g = pixel[1] >> 2;
        let b = pixel[2] >> 3;
        let color: u16 = ((r as u16) << 11) | ((g as u16) << 5) | (b as u16);

        dc_pin.set_high();
        spi.write(&[(color >> 8) as u8, color as u8]).expect("Failed to send pixel data");
    }
}
