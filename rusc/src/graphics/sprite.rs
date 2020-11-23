use crate::graphics::Sampler;
use crate::graphics::Tbin;
use crate::graphics::Tile;
use embedded_graphics::{
  geometry::{Point, Size},
  pixelcolor::Rgb565,
  primitives::Rectangle,
};

// use hifive1::sprintln;

pub struct Sprite<'a> {
  tbin: &'a Tbin<'a>,
  layout: Size,
  tiles: &'a [u8],

  pub bounds: Rectangle,
}

impl<'a> Sprite<'a> {
  pub fn new(tbin: &'a Tbin, layout: Size, tiles: &'a [u8]) -> Sprite<'a> {
    let position = Point::zero();

    let bounds = Rectangle::new(
      position,
      Size::new(
        tbin.tile_size.width * layout.width,
        tbin.tile_size.height * layout.height,
      ),
    );

    Sprite {
      tbin,
      layout,
      tiles,
      bounds,
    }
  }
}

impl<'a> Sampler for Sprite<'a> {
  fn get_pixel(&self, x: u32, y: u32) -> Option<Rgb565> {
    let tile_pixel_width = self.tbin.tile_size.width;
    let tile_pixel_height = self.tbin.tile_size.height;
    let layout_width = self.layout.width;

    let tile_y = y / tile_pixel_height;
    let tile_x = x / tile_pixel_width;

    let tile_pixel_y = y - tile_y * tile_pixel_height;
    let tile_pixel_x = x - tile_x * tile_pixel_width;

    let tile_index = self.tiles[(tile_y * layout_width + tile_x) as usize];

    // self.tbin.tile_pixel(tile_index, tile_pixel_x, tile_pixel_y)
    Tile::new(self.tbin, tile_index).get_pixel(tile_pixel_x, tile_pixel_y)
  }

  fn bounds(&self) -> &Rectangle {
    &self.bounds
  }
}
