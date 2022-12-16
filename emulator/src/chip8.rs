use std::f64;
use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Emulator {
    context: CanvasRenderingContext2d,
}
impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            context: {
                let document = window().unwrap().document().unwrap();
                let canvas_html_element = document
                    .query_selector("canvas")
                    .unwrap()
                    .expect("Canvas not found!");
                let canvas = canvas_html_element
                    .dyn_into::<HtmlCanvasElement>()
                    .expect("Error casting canvas type!");
                canvas
                    .get_context("2d")
                    .unwrap()
                    .expect("Could not get canvas context!")
                    .dyn_into::<CanvasRenderingContext2d>()
                    .expect("Error casting canvas context type!")
            },
        }
    }

    pub fn smiley_face(&self) {
        self.context.begin_path();

        // Draw the outer circle.
        self.context
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        self.context.move_to(110.0, 75.0);
        self.context
            .arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI)
            .unwrap();

        // Draw the left eye.
        self.context.move_to(65.0, 65.0);
        self.context
            .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the right eye.
        self.context.move_to(95.0, 65.0);
        self.context
            .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        self.context.stroke();
    }
}
