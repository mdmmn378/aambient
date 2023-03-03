use display_info::DisplayInfo;
use screenshots::Screen;
use std::{fs, time::Instant};

fn main() {
    let display_infos = DisplayInfo::all().unwrap();
    let display_info = display_infos.first().unwrap();
    println!("{:?}", display_info);

    let screen = Screen::new(&display_info);

    let image = screen.capture_area(0, 0, 300, 300).unwrap();
    let buffer = image.buffer();
    fs::write(format!("./{}-2.png", screen.display_info.id), buffer).unwrap();
}
