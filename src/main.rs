use fishrs::out::lcd::Ili9341Lcd;
use rppal::spi::{Bus, Mode, SlaveSelect};

const LCD_WIDTH: usize = 240;
const LCD_HEIGHT: usize = 320;

fn main() {
    let mut lcd = Ili9341Lcd::new(24, 25, 18, (Bus::Spi0, SlaveSelect::Ss0, 32_000_000, Mode::Mode0), LCD_WIDTH, LCD_HEIGHT).expect("Init lcd");
    lcd.init();

    let img = image::open("splash.jpg").expect("Failed to open image");

    // Resize the image to fit the LCD screen
    let img = img.resize(LCD_WIDTH as u32, LCD_HEIGHT as u32, image::imageops::FilterType::Nearest);

    // Send the image to the LCD
    lcd.send_image(&img.to_rgb8()).expect("Write image");
    loop {

    }
}
