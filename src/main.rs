mod emulator;
mod cartridge;
mod bus;
mod register;

use macroquad::{prelude::*};
use emulator::Emulator;

const SIZE : (i32, i32) = (1920, 1080);

fn window_conf() -> Conf {
    Conf {
        window_title: "RusticBoy".to_owned(),
        window_width : SIZE.0,
        window_height: SIZE.1,
        fullscreen : false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() 
{
    // TEMPORARY GAMEBOY BUFFER
    let buffer = vec![255; 160 * 140 * 4];

    // GAMEBOY RENDER IMAGE
    let gb_image = Image{
        width : 160,
        height : 140,
        bytes : buffer,
    };

    // GAMEBOY TEXTURE
    let gb_texture = Texture2D::from_image(&gb_image);

    // EMULATOR
    let emulator : Emulator = Emulator::init_emulator();



    loop {
        // CLEAR
        clear_background(BLACK);

        // EMULATION CYCLE

        // DRAW
        gb_texture.update(&gb_image);
        draw_texture(gb_texture, 0.0, 0.0, WHITE);


        // CHECK IF ESC
        if is_key_down(KeyCode::Escape)
        {
            break;
        }

        // UPDATE
        next_frame().await
    }
}