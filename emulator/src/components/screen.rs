use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Debug)]
pub struct Screen {
    context: CanvasRenderingContext2d,
}
impl Screen {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub fn init() -> Screen {
        Screen {
            context: {
                let document = window().unwrap().document().unwrap();
                let canvas_html_element = document
                    .query_selector("canvas")
                    .unwrap()
                    .expect("Canvas not found!");
                let canvas = canvas_html_element
                    .dyn_into::<HtmlCanvasElement>()
                    .expect("Error casting canvas type!");

                canvas.set_width(Self::WIDTH as u32);
                canvas.set_height(Self::HEIGHT as u32);

                canvas
                    .get_context("2d")
                    .unwrap()
                    .expect("Could not get canvas context!")
                    .dyn_into::<CanvasRenderingContext2d>()
                    .expect("Error casting canvas context type!")
            },
        }
    }
    pub fn update(&self, gfx: &[bool; Self::HEIGHT * Self::WIDTH]) {
        for row in 0..Self::HEIGHT {
            for col in 0..Self::WIDTH {
                let color = if gfx[row * Self::WIDTH + col] {
                    "#fff"
                } else {
                    "#000"
                };

                self.context.set_fill_style(&color.into());
                self.context.fill_rect(col as f64, row as f64, 1.0, 1.0);
            }
        }
    }
}
