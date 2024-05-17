extern crate raylib;
use raylib::ffi::SetTextureFilter;
use raylib::prelude::*;
// use std::f32::consts::PI;
use rand::random;

mod pallette;

use crate::pallette::*;

fn main() {
    let (screen_width, screen_height) = (320, 240);

    let (window_width, window_height) = (screen_width * 2, screen_height * 2);

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Wave Simulation")
        .build();

    let mut target = rl
        .load_render_texture(&thread, screen_width as u32, screen_height as u32)
        .expect("Failed to load render texture");

    unsafe {
        SetTextureFilter(
            target.texture,
            raylib::consts::TextureFilter::TEXTURE_FILTER_POINT as i32,
        );
    }

    rl.set_target_fps(60);

    let mut palette = make_palette();

    while !rl.window_should_close() {
        let t = (rl.get_time() as f32) * 0.5;

        let mut d0 = rl.begin_drawing(&thread);

        {
            let mut d = d0.begin_texture_mode(&thread, &mut target);

            d.clear_background(Color::ORANGE);

            for fx in 0..screen_width {
                let prev_wave_height = 0.0;
                let mut prev_fy0 = 0;

                for fy in 0..screen_height {
                    // define the vanishing point coordinates
                    let (vp_x, vp_y) = (160.0, 120.5);

                    // define the distance
                    let d1 = 160.0;

                    // calculate the distance from the center
                    let cx = vp_x - fx as f32;
                    let cy = vp_y - fy as f32;

                    // calculate the angle mapping
                    let nx = cx / cy / 2.0;
                    let ny = 320.0 / cy;

                    let dist = (cx * cx + cy * cy).sqrt();

                    // A variable to store the total height
                    let mut wave_height = (40.0 * dist).sin() * f32::max(0.0, t / 2.0 - 40.0);

                    // Calculate the height of the superposition of waves at a given position and time
                    let mut i: f32 = 0.0;

                    // select either water or sky
                    let iterations = if fy < 120 { 4.0 } else { 16.0 };
                    while i < iterations {
                        let amplitude = i / 40.0;
                        let frequency = 2.0 + (i).cos();
                        // dx and dy are the components of the direction of the wave
                        let dx = (i * i).sin();
                        let dy = (i * i * i).cos();
                        let time_shift = t / 14.0 * iterations;
                        wave_height -=
                            amplitude * (frequency * (ny * dy + nx * dx) + time_shift).sin().abs();
                        i += 1.0;
                    }

                    // how big the waves are in time
                    let wave_scale = f32::min(2.0 * t, 40.0);
                    let perspective_height = wave_height * wave_scale * cy / d1;
                    // let perspective_height =
                    //     wave_height * wave_scale * cy / d1 + cx / (80.0 - wave_scale) * t.sin(); // for some waving

                    // how big the waves are in time
                    let h_color = 1.0 - (perspective_height - prev_wave_height).abs() / 6.0;

                    // alien blob/ship
                    let radius = f32::min(2.0 * t - 70.0, 50.0);
                    let blob_color = dist / radius;
                    let color = if dist < radius { blob_color } else { h_color };

                    // add cinematic vignette effect (dist) with a bit of fresnel effect (cy)
                    let p_color = (color + (cy - dist) / 512.0).max(0.0);

                    // set color with slight film grain effect (also to remove color banding)
                    let final_color = (250.0 * p_color + 6.0 * random::<f32>()) as u8 as usize;

                    let fy0 = (fy + perspective_height as i32).min(239).max(0);

                    d.draw_line(fx, prev_fy0, fx, fy0, palette[final_color]);
                    prev_fy0 = fy0;
                }

                // set ocean palette with a bit of yellow tint

                let index = (fx as i32) % 128;
                palette[index as usize] =
                    Color::new((2 * index / 4) as u8, 0, (3 * index / 4) as u8, 255);
                palette[(index + 128) as usize] = Color::new(
                    (4 * index / 2) as u8,
                    (64 + 3 * index / 2) as u8,
                    (96 + 2 * index / 2) as u8,
                    255,
                );

                /*
                    let index: u32 = (fx as u32) % 128;
                    palette[index as usize] = Color::get_color(0x03020000 * (index / 4) + 0xff);
                    palette[(index + 128) as usize] =
                        Color::get_color(0x02030400 * (index / 2) + 0x604000ff);
                */

                /*
                    let inline index = (fx as i32) % 128;
                    let inline i = 4*index;
                    i!0x13000 =  0x030200*(index/4);
                    i!0x13200 =  0x020304*(index/2)+0x604000;
                */
            }
            /*
            for y in 0..=15 {
                for x in 0..=15 {
                    d.draw_circle(x * 16, y * 16, 8.0, palette[(y * 16 + x) as usize])
                }
            }
            */
        }

        d0.draw_texture_pro(
            &target.texture(),
            Rectangle::new(
                0.0,
                0.0,
                target.texture().width() as f32,
                -target.texture().height() as f32,
            ),
            Rectangle::new(0.0, 0.0, window_width as f32, window_height as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
    }
}
