use crate::graphics::Sampler;
use embedded_graphics::{
  geometry::Point,
  pixelcolor::Rgb565,
  primitives::{ContainsPoint, Rectangle},
};

pub struct StackLayer<'a>(pub &'a dyn Sampler);

impl<'a> Sampler for StackLayer<'a> {
  fn get_pixel(&self, x: u32, y: u32) -> Option<Rgb565> {
    self.0.get_pixel(x, y)
  }

  fn bounds(&self) -> &Rectangle {
    self.0.bounds()
  }
}

pub struct Stack<'a> {
  layers: &'a [&'a StackLayer<'a>],
  bounds: Rectangle,
}

impl<'a> Stack<'a> {
  pub fn new(layers: &'a [&'a StackLayer<'a>], bounds: Rectangle) -> Stack<'a> {
    Stack { layers, bounds }
  }
}

impl<'a> Sampler for Stack<'a> {
  fn bounds(&self) -> &Rectangle {
    &self.bounds
  }

  fn get_pixel(&self, x: u32, y: u32) -> Option<Rgb565> {
    let mut iterator = self.layers.into_iter();
    let point = Point::new(x as i32, y as i32);

    while let Some(sampler) = iterator.next() {
      if sampler.bounds().contains(point) {
        let local_x = x - sampler.bounds().top_left.x.min(x as i32) as u32;
        let local_y = y - sampler.bounds().top_left.y.min(y as i32) as u32;

        let color = match sampler.get_pixel(local_x, local_y) {
          Some(color) => Some(color),
          None => continue,
        };

        return color;
      }
    }

    None
  }
}
