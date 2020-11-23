use hifive1::hal::prelude::*;

use hifive1::{
  hal::{clock::Clocks, delay::Sleep, DeviceResources},
  pin,
};

pub struct HiFive1Board {
  pub clocks: Clocks,
  pub sleep: Sleep,
}

impl HiFive1Board {
  pub fn new() -> HiFive1Board {
    let resources = HiFive1Board::steal_device_resources();
    let peripherals = resources.peripherals;
    let clint = resources.core_peripherals.clint;

    // Configure clocks
    let clocks = hifive1::clock::configure(peripherals.PRCI, peripherals.AONCLK, 320.mhz().into());

    HiFive1Board {
      sleep: Sleep::new(clint.mtimecmp, clocks),
      clocks,
    }
  }

  pub fn steal_device_resources() -> DeviceResources {
    unsafe { DeviceResources::steal() }
  }

  pub fn configure_uart_for_stdout(&self) {
    let resources = HiFive1Board::steal_device_resources();
    let clocks = self.clocks;
    let peripherals = resources.peripherals;
    let pins = resources.pins;

    hifive1::stdout::configure(
      peripherals.UART0,
      pin!(pins, uart0_tx),
      pin!(pins, uart0_rx),
      115_200.bps(),
      clocks,
    );
  }
}
