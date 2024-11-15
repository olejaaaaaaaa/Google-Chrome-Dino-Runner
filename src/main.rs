#![allow(warnings)]
#![feature(duration_millis_float)]


use std::thread;
use std::time::{Duration, Instant};

use image::GenericImageView;
use screenshots::*;

use enigo::{self, Enigo, Key, Keyboard, Settings};

use enigo::Direction::{Press, Release};
use enigo::Direction::Click;

fn main() {



    let mut enigo = Enigo::new(&Settings::default()).unwrap();



    enigo.key(Key::Super, Press);
    enigo.key(Key::R, Press);
    thread::sleep(Duration::from_secs_f32(1.5));
    enigo.text("chrome https://dino-chrome.com/ru");
    enigo.key(Key::Execute, Press);

    enigo.key(Key::Super, Release);
    enigo.key(Key::R, Release);

    let screen = Screen::all().unwrap()[0];

    thread::sleep(Duration::from_secs_f32(4.0));
    enigo.key(Key::Space, Click);

    let mut time = Instant::now();
    let mut offset_x = 0;

    loop {

        //if time.elapsed().as_millis_f32() > 3.5 {
        //    offset_x += 1;
        //    time = Instant::now();
        //                                    }

        let image = screen.capture_area(500 + offset_x   , 350, 100, 100).unwrap();
        image.save(format!("target/{}.png", "da"));

        
        if image.get_pixel(57, 79).0 != [255, 255, 255, 255] {
            enigo.key(Key::Space, Click);                       
        }
    
    }        

}                                                          
                                     