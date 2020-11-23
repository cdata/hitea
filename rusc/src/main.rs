#![no_std]
#![no_main]

mod graphics;
mod hardware;

extern crate panic_halt;

use embedded_graphics::prelude::*;
use embedded_graphics::{
  geometry::Size, pixelcolor::Rgb565, primitives::Rectangle, style::PrimitiveStyle,
};

use crate::graphics::Pattern;
use crate::graphics::PixelSource;
use crate::graphics::Sprite;
use crate::graphics::Tbin;
use crate::graphics::{Stack, StackLayer};

use crate::hardware::DeviceIntegration;

#[cfg(all(feature = "hifive1_board", feature = "st7735_display"))]
use hifive1::sprintln;

#[cfg(feature = "hifive1_board")]
use crate::hardware::HiFive1Board;

#[cfg(feature = "hifive1_board")]
use riscv_rt::entry;

#[cfg(feature = "st7735_display")]
use crate::hardware::ST7735Display;

#[cfg(all(feature = "hifive1_board", feature = "st7735_display"))]
#[entry]
fn main() -> ! {
  let board = HiFive1Board::new();

  board.configure_uart_for_stdout();

  sprintln!("Starting TFT program...");

  let mut display = ST7735Display::attach_to(&board);

  sprintln!("Configure pins...");

  let viewport = Rectangle::new(Point::zero(), Size::new(128, 128));
  let clear = PrimitiveStyle::with_fill(Rgb565::WHITE);
  let black = PrimitiveStyle::with_fill(Rgb565::BLACK);

  viewport
    .into_styled(clear)
    .draw(&mut display.driver)
    .unwrap();
  viewport
    .into_styled(black)
    .draw(&mut display.driver)
    .unwrap();

  let tbin = Tbin::new(include_bytes!("../assets/sprites.tbin")).unwrap();

  let mut sprite = &mut Sprite::new(&tbin, Size::new(2, 2), &[10, 11, 18, 19]);
  let flower_sprite = Sprite::new(&tbin, Size::new(1, 1), &[33]);
  let mut pattern = &mut Pattern::new(&flower_sprite, viewport);

  sprintln!("Hello RUSC!");

  let mut offset = Point::new(1, 1);
  let viewport_bottom_right = viewport.bounding_box().bottom_right().unwrap();

  loop {
    let top = sprite.bounds.top_left.y;
    let left = sprite.bounds.top_left.x;
    let bottom = top + sprite.bounds.size.height as i32;
    let right = left + sprite.bounds.size.width as i32;

    if right > viewport_bottom_right.x || left < viewport.top_left.x {
      offset.x *= -1;
    }

    if bottom > viewport_bottom_right.y || top < viewport.top_left.y {
      offset.y *= -1;
    }

    sprite.bounds.top_left.x += offset.x * 3;
    sprite.bounds.top_left.y += offset.y * 2;

    pattern.offset.x += 1;
    pattern.offset.y += 1;

    PixelSource(
      &Stack::new(&[&StackLayer(sprite), &StackLayer(pattern)], viewport),
      None,
    )
    .draw(&mut display.driver)
    .unwrap();

    continue;
  }
}
