#![cfg(feature = "for_web_sys")]

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

mod draw_encounter;
mod palette_encounter;

use ilmenit_encounter_rust::draw_encounter::*;
use ilmenit_encounter_rust::palette_encounter::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Initialize the console logger for better debugging.
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    info!("HUHU!");

    // Get the window and document objects.
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

    // Get the canvas rendering context.
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Set the canvas dimensions.
    let screen_width = 320;
    let screen_height = 240;
    let view_scale = 2;
    canvas.set_width((screen_width * view_scale) as u32);
    canvas.set_height((screen_height * view_scale) as u32);

    // Create the palette.
    let palette: Vec<_> = get_palette()
        .iter()
        .map(|&color| {
            let color_shifted = (color << 8) + 0xff;
            JsValue::from_str(&format!("#{:06x}", color_shifted))
        })
        .collect();

    // Draw function.
    let draw_line = move |start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: usize| {
        context.set_stroke_style(&palette[color]);
        context.begin_path();
        context.move_to(start_x as f64, start_y as f64);
        context.line_to(end_x as f64, end_y as f64);
        context.stroke();
    };

    // Animation loop.
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let t = window.performance().unwrap().now() as f32 / 1000.0;
        draw_encounter(screen_width, screen_height, t, &mut draw_line);

        // Schedule ourself for another requestAnimationFrame callback.
        window
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut()>));

    window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}
