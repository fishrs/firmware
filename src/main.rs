use std::{error::Error, io::{stdin, stdout, Read, Write}, thread, time::Duration};

use fishrs::{fish::FishRs, inp::reader::DataReader, out::{lcd::Ili9341Lcd, motor_driver::MotorDriver}};
use rppal::{pwm::Channel, spi::{Bus, Mode, SlaveSelect}, uart::Parity};

const LCD_WIDTH: usize = 240;
const LCD_HEIGHT: usize = 320;

pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

pub fn draw_img(lcd: &mut Ili9341Lcd, img_str: &str) {
    let img = image::open(img_str).expect("Failed to open image");

    // Resize the image to fit the LCD screen
    let img = img.resize(LCD_WIDTH as u32, LCD_HEIGHT as u32, image::imageops::FilterType::Nearest);

    // Send the image to the LCD
    lcd.send_image(&img.to_rgb8()).expect("Write image");
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lcd = Ili9341Lcd::new(24, 25, 18, (Bus::Spi0, SlaveSelect::Ss0, 32_000_000, Mode::Mode0), LCD_WIDTH, LCD_HEIGHT).expect("Init lcd");
    lcd.init();

    draw_img(&mut lcd, "splash.jpg");

    let mut servo = MotorDriver::new(Channel::Pwm1);

    loop {
        servo.set_angle(120f64);
        thread::sleep(Duration::from_millis(5000));

        servo.set_angle(25f64);
        thread::sleep(Duration::from_millis(5000));
    
        pause();
        thread::sleep(Duration::from_millis(5000));

    }

    /*loop {
        for i in 1..=3_659 {
            draw_img(&mut lcd, &format!("frames/subway{:04}.jpg", i));
        }
    }*/

    /*let reader = DataReader::new((9600, Parity::None, 8, 1), 5/*Change*/)?;

    //let mut data_log = vec![];

    //Initialize model

    loop {

        //let new_state = reader.read();
        //data_log.push(new_state);


        let save_flag = false;

        if save_flag {
            //Serialize LakeData and append it to the end of the data file if save button is being held
        }


        //Check current state. If in infer mode use network to determine if we should reel
        

        std::thread::sleep(Duration::from_millis(500))
    }*/
}
