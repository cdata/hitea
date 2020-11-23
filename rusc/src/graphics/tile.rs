use crate::graphics::Sampler;
use crate::graphics::Tbin;

use embedded_graphics::{geometry::Point, pixelcolor::Rgb565, primitives::Rectangle};

pub struct Tile<'a> {
  pub tbin: &'a Tbin<'a>,
  pub index: u8,
  pub bounds: Rectangle,
}

impl<'a> Tile<'a> {
  pub fn new(tbin: &'a Tbin, index: u8) -> Tile<'a> {
    let bounds = Rectangle::new(Point::zero(), tbin.tile_size.clone());

    Tile {
      tbin,
      index,
      bounds,
    }
  }
}

impl<'a> Sampler for Tile<'a> {
  fn bounds(&self) -> &Rectangle {
    &self.bounds
  }

  fn get_pixel(&self, x: u32, y: u32) -> Option<Rgb565> {
    self.tbin.tile_pixel(self.index, x, y)
  }
}
