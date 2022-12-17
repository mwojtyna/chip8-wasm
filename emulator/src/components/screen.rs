use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Debug)]
pub struct Screen {
    display: CanvasRenderingContext2d,
    display_width: u32,
    display_height: u32,
}
impl Screen {
    pub fn init(width: u32, height: u32) -> Screen {
        Screen {
            display_width: width,
            display_height: height,
            display: {
                let document = window().unwrap().document().unwrap();
                let canvas_html_element = document
                    .query_selector("canvas")
                    .unwrap()
                    .expect("Canvas not found!");
                let canvas = canvas_html_element
                    .dyn_into::<HtmlCanvasElement>()
                    .expect("Error casting canvas type!");

                canvas.set_width(width);
                canvas.set_height(height);

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
        for x in 0..self.display_width {
            for y in 0..self.display_height {
                let color = (x as f32) * (y as f32)
                    / ((self.display_width - 1) as f32 * (self.display_height - 1) as f32)
                    * 255.0;

                self.display
                    .set_fill_style(&format!("rgb({}, {}, {})", color, color, color).into());
                self.display.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
}
