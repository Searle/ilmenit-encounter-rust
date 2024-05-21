#![cfg(feature = "for_raylib")]

extern crate raylib;
use raylib::prelude::*;

use ilmenit_encounter_rust::draw_encounter::*;
use ilmenit_encounter_rust::palette_encounter::*;

pub fn main() {
    let screen_width = 320;
    let screen_height = 240;
    let view_scale = 2;

    let (window_width, window_height) = (screen_width * view_scale, screen_height * view_scale);

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Ilmenit Encounter ported to Rust")
        .build();

    let mut target = rl
        .load_render_texture(&thread, screen_width as u32, screen_height as u32)
        .expect("Failed to load render texture");
    target
        .texture()
        .set_texture_filter(&thread, raylib::consts::TextureFilter::TEXTURE_FILTER_POINT);

    rl.set_target_fps(60);

    let palette = get_palette().map(|color| Color::get_color((color << 8) + 0xff));

    while !rl.window_should_close() {
        let t = rl.get_time() as f32;

        let mut d0 = rl.begin_drawing(&thread);
        {
            let mut d = d0.begin_texture_mode(&thread, &mut target);

            let mut draw_line = |start_x, start_y, end_x, end_y, color| {
                d.draw_line(start_x, start_y, end_x, end_y, palette[color]);
            };

            draw_encounter(screen_width, screen_height, t, &mut draw_line);
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
