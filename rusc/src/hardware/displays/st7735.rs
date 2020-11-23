#[cfg(feature = "hifive1_board")]
use hifive1::hal::prelude::*;

#[cfg(feature = "hifive1_board")]
use crate::hardware::{DeviceIntegration, HiFive1Board};
#[cfg(feature = "hifive1_board")]
use embedded_hal::{blocking::spi, digital::v2::OutputPin};
#[cfg(feature = "hifive1_board")]
use hifive1::{
  hal::delay::Sleep,
  hal::e310x::QSPI1,
  hal::gpio::gpio0::{Pin0, Pin1, Pin2, Pin3, Pin4, Pin5},
  hal::gpio::{NoInvert, Output, Regular, IOF0},
  hal::spi::{Spi, MODE_0},
  pin,
};

use st7735_lcd::{Orientation, ST7735};

pub struct ST7735Display<T, U, V>
where
  T: spi::Write<u8, Error = core::convert::Infallible>,
  U: OutputPin<Error = core::convert::Infallible>,
  V: OutputPin<Error = core::convert::Infallible>,
{
  pub driver: ST7735<T, U, V>,
}

impl<T, U, V> ST7735Display<T, U, V>
where
  T: spi::Write<u8, Error = core::convert::Infallible>,
  U: OutputPin<Error = core::convert::Infallible>,
  V: OutputPin<Error = core::convert::Infallible>,
{
  pub fn new(mut driver: ST7735<T, U, V>) -> ST7735Display<T, U, V> {
    driver.set_orientation(&Orientation::Landscape).unwrap();
    driver.set_offset(1, 2);

    ST7735Display { driver }
  }
}

#[cfg(feature = "hifive1_board")]
impl DeviceIntegration<HiFive1Board>
  for ST7735Display<
    Spi<
      QSPI1,
      (
        Pin3<IOF0<NoInvert>>,
        Pin4<IOF0<NoInvert>>,
        Pin5<IOF0<NoInvert>>,
        Pin2<IOF0<NoInvert>>,
      ),
    >,
    Pin0<Output<Regular<NoInvert>>>,
    Pin1<Output<Regular<NoInvert>>>,
  >
{
  fn attach_to(
    host: &'_ HiFive1Board,
  ) -> ST7735Display<
    Spi<
      QSPI1,
      (
        Pin3<IOF0<NoInvert>>,
        Pin4<IOF0<NoInvert>>,
        Pin5<IOF0<NoInvert>>,
        Pin2<IOF0<NoInvert>>,
      ),
    >,
    Pin0<Output<Regular<NoInvert>>>,
    Pin1<Output<Regular<NoInvert>>>,
  > {
    let resources = HiFive1Board::steal_device_resources();

    let pins = resources.pins;
    let peripherals = resources.peripherals;

    // Configure SPI pins
    let mosi = pin!(pins, spi0_mosi).into_iof0();
    let miso = pin!(pins, spi0_miso).into_iof0();
    let sck = pin!(pins, spi0_sck).into_iof0();
    let cs = pin!(pins, spi0_ss0).into_iof0();

    let dc = pin!(pins, dig8).into_output();
    let rst = pin!(pins, dig9).into_output();

    // Configure SPI
    let pins = (mosi, miso, sck, cs);
    let spi = Spi::new(
      peripherals.QSPI1,
      pins,
      MODE_0,
      16_000_000.hz(),
      host.clocks,
    );

    let mut driver = ST7735::new(spi, dc, rst, false, false, 128, 128);
    let clint = resources.core_peripherals.clint;

    driver
      .init(&mut Sleep::new(clint.mtimecmp, host.clocks))
      .unwrap();

    ST7735Display::new(driver)
  }
}
