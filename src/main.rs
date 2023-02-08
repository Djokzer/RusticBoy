use macroquad::{prelude::*, miniquad::conf, window};

const SIZE : u16 = 1024;

fn window_conf() -> Conf {
    Conf {
        window_title: "RusticBoy".to_owned(),
        window_height: SIZE as i32,
        window_width : SIZE as i32,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() 
{
    loop {
        //CLEAR
        clear_background(BLACK);

        next_frame().await
    }
}