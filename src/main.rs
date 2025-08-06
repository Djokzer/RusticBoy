mod emulator;
mod cartridge;
mod bus;
mod register;
mod cpu;

use std::time::{SystemTime, Duration};
use macroquad::prelude::*;
use emulator::Emulator;

const SIZE : (i32, i32) = (160, 140);

fn window_conf() -> Conf 
{
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
    let mut gb_emulator : Emulator = Emulator::init_emulator();
    if !gb_emulator.load_boot_rom("roms/dmg_boot.bin")  // "roms/dmg_boot.bin"
    {
        gb_emulator.init_emulator_without_bootrom(); // SKIP ROM BOOT
    }
    gb_emulator.load_rom("roms/tetris.gb"); // LOAD ROM

    // CLOCK    
    const CLOCK_SPEED: u32 = 4_194_304;   // Hz
    const CYCLES_PER_FRAME: u32 = 70224;  // (CLOCK SPEED / REFRESH RATE)
    const TARGET_WAIT_TIME: u64 = 1666666667;
    let mut cycles : u32 = 0;
    let mut start_time = SystemTime::now();

    // CLEAR SCREEN
    clear_background(BLACK);
    loop 
    {
        // EMULATION CYCLE
        cycles += gb_emulator.emulation_cycle();

        if cycles >= CYCLES_PER_FRAME
        {
            cycles = 0;

            // RENDER
            gb_texture.update(&gb_image);
            draw_texture(gb_texture, 0.0, 0.0, WHITE);


            // WAIT
            let elapsed_time = start_time.elapsed().unwrap().as_nanos() as u64;
            if elapsed_time < TARGET_WAIT_TIME
            {
                let sleep_time = Duration::from_nanos(TARGET_WAIT_TIME - elapsed_time);
                std::thread::sleep(sleep_time);
            }
            start_time = SystemTime::now(); 

            // UPDATE
            next_frame().await
        } 

        // CHECK IF ESC
        if is_key_down(KeyCode::Escape)
        {
            break;
        }
    }
}
