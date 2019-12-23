use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// #[wasm_bindgen]
pub struct Canvas {
  pub canvas: web_sys::HtmlCanvasElement,
  pub ctx: web_sys::CanvasRenderingContext2d,
  scaled_width: u32,
  scaled_height: u32,
  width: u32,
  height: u32,
}

// #[wasm_bindgen]
impl Canvas {
  pub fn new(attr_id: &str, width: u32, height: u32) -> Canvas {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(attr_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .map_err(|_| ())
      .unwrap();

    let ctx = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();

    let scaled_width = canvas.width() / width;
    let scaled_height = canvas.height() / height;

    Canvas {
      canvas,
      ctx,
      scaled_width,
      scaled_height,
      width,
      height,
    }
  }

  pub fn draw(&self, x: u32, y: u32, color: &str) {
    assert!(x < self.width);
    assert!(y < self.height);

    self.ctx.set_fill_style(&JsValue::from_str(color));
    let x = x * self.scaled_width;
    let y = y * self.scaled_height;

    self.ctx.fill_rect(
      f64::from(x),
      f64::from(y),
      f64::from(self.scaled_width),
      f64::from(self.scaled_height),
    );
  }

  pub fn clear_all(&self) {
    self.ctx.set_fill_style(&JsValue::from_str("black"));
    self.ctx.fill_rect(
      0.0,
      0.0,
      f64::from(self.width * self.scaled_width),
      f64::from(self.height * self.scaled_height),
    );
    // TODO: look for a better implementation:
    // self.ctx.fill_rect(
    //   0.0,
    //   0.0,
    //   f64::from(self.canvas.width()),
    //   f64::from(self.canvas.height()),
    // );
  }
}
