// use crate::tbin::tile::Tile;

use embedded_graphics::{
  geometry::Size,
  pixelcolor::{raw::RawU16, Rgb565},
};

// use hifive1::sprintln;

const MAGIC: [u8; 4] = [0xCA, 0xFE, 0xF0, 0x0D];
const VERSION: u8 = 0;
const HEADER_BYTES: u8 = 10;
const COLOR_PALETTE_BYTES: u8 = 32;
const TILE_REMAP_BYTES: u8 = 2;
const TILE_HEADER_BYTES: u8 = 1;
const COLOR_BYTES: u8 = 2;

pub struct Tbin<'a> {
  data: &'a [u8],
  remap_offset: u8,
  tile_offset: u8,

  pub remap_count: &'a u8,
  pub tile_size: Size,
  pub color_palette_count: &'a u8,
  pub rows: u8,
  pub columns: u8,
}

impl<'a> Tbin<'a> {
  pub fn new(data: &'a [u8]) -> Option<Tbin<'a>> {
    // Check magic
    for index in 0..=3 {
      if data[index] != MAGIC[index] {
        return None;
      }
    }

    // Check version
    if data[4] != VERSION {
      return None;
    }

    let sheet_size = &data[5];
    let tile_size = &data[6];
    let color_palette_count = &data[7];
    let remap_count = &data[8];

    let remap_offset = HEADER_BYTES + color_palette_count * COLOR_PALETTE_BYTES;
    let tile_offset = remap_offset + remap_count * TILE_REMAP_BYTES;

    Some(Tbin {
      data,
      tile_size: Size::new(
        ((tile_size >> 4) + 1) as u32,
        ((tile_size & 0xf) + 1) as u32,
      ),
      rows: (sheet_size >> 4) + 1,
      columns: (sheet_size & 0xf) + 1,
      color_palette_count,
      remap_count,
      remap_offset,
      tile_offset,
    })
  }

  pub fn palette_color(&self, palette_index: u8, color_index: u8) -> Rgb565 {
    let data_offset =
      (HEADER_BYTES + palette_index * COLOR_PALETTE_BYTES + color_index * COLOR_BYTES) as usize;

    let color_high = self.data[data_offset] as u16;
    let color_low = self.data[data_offset + 1] as u16;

    let color: u16 = (color_high << 8) | color_low;

    Rgb565::from(RawU16::new(color))
  }

  fn sparse_tile_index(&self, tile_index: u8) -> Option<u8> {
    let mut original_index_remapped = false;

    let remap_count = *self.remap_count;

    for remap_index in 0..remap_count {
      let remap_start = (self.remap_offset + remap_index * TILE_REMAP_BYTES) as usize;
      let remap_end = remap_start + 1;

      if let [remap_from, remap_to] = self.data[remap_start..=remap_end] {
        if remap_from == tile_index {
          return Some(remap_to);
        } else if remap_to == tile_index {
          original_index_remapped = true;
        }
      }
    }

    if !original_index_remapped {
      return Some(tile_index);
    }

    None
  }

  pub fn tile_pixel(&self, tile_index: u8, x: u32, y: u32) -> Option<Rgb565> {
    let tile_bytes = TILE_HEADER_BYTES + (self.tile_size.width * self.tile_size.height / 2) as u8;

    if let Some(actual_tile_index) = self.sparse_tile_index(tile_index) {
      let tile_offset = self.tile_offset as usize + (actual_tile_index * tile_bytes) as usize;

      let palette_setting = self.data[tile_offset];
      let palette_index = palette_setting >> 4;
      let transparent_color_index = palette_setting & 0xf;

      let pixel_index = y * self.tile_size.width + x;
      let pixel_byte_index = tile_offset + (pixel_index as usize) / 2 + TILE_HEADER_BYTES as usize;

      let color_index = match pixel_index % 2 {
        0 => self.data[pixel_byte_index] >> 4,
        1 => self.data[pixel_byte_index] & 0xf,
        _ => transparent_color_index,
      };

      if color_index != transparent_color_index {
        return Some(self.palette_color(palette_index, color_index));
      }
    }

    None
  }
}
