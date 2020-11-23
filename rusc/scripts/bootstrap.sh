#!/bin/bash

RISCV64_TOOLCHAIN=riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14
JLINK_DEB_URL=https://www.segger.com/downloads/jlink/JLink_Linux_x86_64.deb

rustup target add riscv32imac-unknown-none-elf

if [ ! -d ./vendor/riscv64_toolchain ]; then
  mkdir -p ./vendor
  pushd ./vendor
  wget https://static.dev.sifive.com/dev-tools/$RISCV64_TOOLCHAIN.tar.gz
  tar -xvf ./$RISCV64_TOOLCHAIN.tar.gz
  ln -s ./$RISCV64_TOOLCHAIN ./riscv64_toolchain
  rm ./$RISCV64_TOOLCHAIN.tar.gz
  popd
fi

if [[ `which JLinkGDBServer` == '' ]]; then
  echo "The JLink debugger package is required for debugging a HiFive1 Rev B"
  echo "Opening JLink download page for manual installation"
  xdg-open https://www.segger.com/downloads/jlink/JLink_Linux_x86_64.deb
fi

echo "Done!"