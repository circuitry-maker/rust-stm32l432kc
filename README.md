# rust-stm32l432kc
STM32L432KC board examples in rust

## Compilation and load

*prerequisites*
```console
apt-get install binutils-arm-none-eabi
apt-get install openocd
apt-get install gdb-multiarch
rustup target add thumbv7em-none-eabihf
rustup component add clippy
cargo install cargo-binutils
rustup component add llvm-tools-preview
```
Udev rules for Black Magic Probe on an STM32
```
# UDEV Rules for Black Magic Probe STM32
# copy this file to /etc/udev/rules.d/50-usb-stlink.rules

# reload rules:
#   $ sudo udevadm control --reload-rules && sudo udevadm trigger
SUBSYSTEMS=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="6018", GROUP="plugdev", MODE="0666"
```

If you have different programmer you should change idProduct and idVendor. To find out your numbers first plug your USB device and then type lsusb:
```console
~$ lsusb
Bus 001 Device 026: ID 0483:374b STMicroelectronics ST-LINK/V2.1
```

*build*
``` console
$ cargo build
$ cargo build --release
$ cargo build --target thumbv7em-none-eabihf --release  
```

*check*
```
$ arm-none-eabi-readelf -h <output>
$ arm-none-eabi-readelf -A <output>
$ cargo objdump --release -- --disassemble --no-show-raw-insn
$ arm-none-eabi-size -tA <output>
```

*elf to bin/hex format*
``` console
$ arm-none-eabi-objcopy -O ihex <output> <output>.hex 
$ arm-none-eabi-objcopy -O binary <output> <output>.bin 
```

*load*
``` console
$ cp <output>.bin <device-path>
$ cp <output>.bin /run/media/luca/NODE_L432KC
```

*debug with openocd*
in one console:
``` console
$ openocd -f nucleo_l432kc.cfg
```

in another console:
``` console
$ gdb-multiarch -q -x openocd.gdb <output>
```
or
``` console
$ gdb -q -x openocd.gdb <output>
```

*read from serial interface via USB*
``` console
$ tail -f /dev/ttyACM1
$ screen /dev/ttyACM1 9600
```

![](docs/nucleo_l432kc.png)


Documentation:
- [The Embedded Rust Book](https://doc.rust-lang.org/stable/embedded-book/)
- [stm32l4xx-hal](https://github.com/stm32-rs/stm32l4xx-hal)