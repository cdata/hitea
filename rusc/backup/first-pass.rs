#![no_std]
#![no_main]

mod graphics;

extern crate panic_halt;

use embedded_graphics::prelude::*;
use embedded_graphics::{
  geometry::Size, pixelcolor::Rgb565, primitives::Rectangle, style::PrimitiveStyle,
};

use st7735_lcd;
use st7735_lcd::Orientation;

use hifive1::hal::prelude::*;
use hifive1::hal::spi::{Spi, MODE_0};
use hifive1::hal::DeviceResources;
use hifive1::{pin, sprintln};

use hifive1::hal::delay::Delay;
use hifive1::hal::delay::Sleep;

use riscv_rt::entry;

use crate::graphics::Pattern;
use crate::graphics::PixelSource;
use crate::graphics::Sampler;
use crate::graphics::Sprite;
use crate::graphics::Tbin;
use crate::graphics::{Stack, StackLayer};
// use crate::graphics::Tile;

#[entry]
fn main() -> ! {
  sprintln!("Starting TFT program...");

  let resources = DeviceResources::take().unwrap();
  let peripherals = resources.peripherals;
  let pins = resources.pins;

  // Configure clocks
  let clocks = hifive1::clock::configure(peripherals.PRCI, peripherals.AONCLK, 320.mhz().into());

  // Configure UART for stdout
  // hifive1::stdout::configure(
  //   peripherals.UART0,
  //   pin!(pins, uart0_tx),
  //   pin!(pins, uart0_rx),
  //   115_200.bps(),
  //   clocks,
  // );

  sprintln!("Configure pins...");

  // Configure SPI pins
  let mosi = pin!(pins, spi0_mosi).into_iof0();
  let miso = pin!(pins, spi0_miso).into_iof0();
  let sck = pin!(pins, spi0_sck).into_iof0();
  let cs = pin!(pins, spi0_ss0).into_iof0();

  let dc = pin!(pins, dig8).into_output();
  let rst = pin!(pins, dig9).into_output();

  // Configure SPI
  let pins = (mosi, miso, sck, cs);
  let spi = Spi::new(peripherals.QSPI1, pins, MODE_0, 16_000_000.hz(), clocks);

  let clint = resources.core_peripherals.clint;
  let mut sleep = Sleep::new(clint.mtimecmp, clocks);

  sprintln!("Initialize display...");

  let mut display = st7735_lcd::ST7735::new(spi, dc, rst, false, false, 128, 128);

  display.init(&mut sleep).unwrap();
  display.set_orientation(&Orientation::Landscape).unwrap();
  display.set_offset(1, 2);

  let viewport = Rectangle::new(Point::zero(), Size::new(128, 128));
  let clear = PrimitiveStyle::with_fill(Rgb565::WHITE);
  let black = PrimitiveStyle::with_fill(Rgb565::BLACK);
  // let fill = PrimitiveStyle::with_fill(Rgb565::RED);

  viewport.into_styled(clear).draw(&mut display).unwrap();
  viewport.into_styled(black).draw(&mut display).unwrap();

  let tbin = Tbin::new(include_bytes!("../assets/sprites.tbin")).unwrap();

  let mut sprite = &mut Sprite::new(&tbin, Size::new(2, 2), &[10, 11, 18, 19]);
  let flower_sprite = Sprite::new(&tbin, Size::new(1, 1), &[33]);
  let mut pattern = &mut Pattern::new(&flower_sprite, viewport);

  // let samplers = &[&StackLayer(sprite), &StackLayer(pattern)];

  // let stack = Stack::new(samplers, viewport);

  sprintln!("Drawing sprite...");

  // sprite.draw(&mut display).unwrap();

  // PixelSource(pattern, None).draw(&mut display).unwrap();

  sprintln!("Hello RUSC!");

  let mut offset = Point::new(1, 1);
  let viewport_bottom_right = viewport.bounding_box().bottom_right().unwrap();
  let mut frame = 0;

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
    .draw(&mut display)
    .unwrap();

    // match frame % 3 {
    //   0 => viewport.into_styled(clear).draw(&mut display).unwrap(),
    //   1 => viewport.into_styled(black).draw(&mut display).unwrap(),
    //   2 => PixelSource(
    //     &Stack::new(&[&StackLayer(sprite), &StackLayer(pattern)], viewport),
    //     None,
    //   )
    //   .draw(&mut display)
    //   .unwrap(),
    //   _ => continue,
    // }

    frame += 1;

    continue;
  }
}
