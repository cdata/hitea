use crate::graphics::Sampler;

use embedded_graphics::{geometry::Point, pixelcolor::Rgb565, primitives::Rectangle};

pub struct Pattern<'a, T>
where
  T: Sampler,
{
  pub offset: Point,
  sampler: &'a T,
  bounds: Rectangle,
}

impl<'a, T> Pattern<'a, T>
where
  T: Sampler,
{
  pub fn new(sampler: &'a T, bounds: Rectangle) -> Pattern<'a, T> {
    Pattern {
      sampler,
      offset: Point::zero(),
      bounds,
    }
  }
}

impl<'a, T> Sampler for Pattern<'a, T>
where
  T: Sampler,
{
  fn bounds(&self) -> &Rectangle {
    &self.bounds
  }

  fn get_pixel(&self, x: u32, y: u32) -> Option<Rgb565> {
    let offset_x = (x + self.offset.x as u32) % self.bounds.size.width;
    let offset_y = (y + self.offset.y as u32) % self.bounds.size.height;

    let sampler_bounds = self.sampler.bounds();
    let sprite_width = sampler_bounds.size.width;
    let sprite_height = sampler_bounds.size.height;

    let pattern_column = offset_x / sprite_width;
    let pattern_row = offset_y / sprite_height;

    let local_pixel_x = offset_x - pattern_column * sprite_width;
    let local_pixel_y = offset_y - pattern_row * sprite_height;

    self.sampler.get_pixel(local_pixel_x, local_pixel_y)
  }
}
