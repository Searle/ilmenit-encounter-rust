use rand::random;

pub fn draw_encounter(
    screen_width: i32,
    screen_height: i32,
    t: f32,
    draw_line: &mut dyn FnMut(i32, i32, i32, i32, usize),
) {
    for fx in 0..screen_width {
        let mut prev_wave_height = 0.0;

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

            // Draw lines also for blob to imitate reflection
            draw_line(
                fx + 1,
                fy + perspective_height.ceil() as i32,
                fx + 1,
                fy + prev_wave_height.floor() as i32,
                final_color,
            );

            prev_wave_height = perspective_height;
        }
    }
}
