use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Debug)]
pub struct Screen {
    display: CanvasRenderingContext2d,
}
impl Screen {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub fn init() -> Screen {
        Screen {
            display: {
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

    pub fn test_display(&self) {
        for x in 0..Self::WIDTH {
            for y in 0..Self::HEIGHT {
                let color = (x as f32) * (y as f32)
                    / ((Self::WIDTH - 1) as f32 * (Self::HEIGHT - 1) as f32)
                    * 255.0;

                self.display
                    .set_fill_style(&format!("rgb({}, {}, {})", color, color, color).into());
                self.display.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
}
