BINARY=stm

TARGET=thumbv7em-none-eabihf
TARGET_FOLDER=target
LEVEL=release
STM_PATH=/run/media/luca/NODE_L432KC

EL=arm-none-eabi-readelf
CP=arm-none-eabi-objcopy
OD=arm-none-eabi-objdump
SE=arm-none-eabi-size

.PHONY: build

clean:
	cargo clean && rm -f $(TARGET_FOLDER)
	rm -f *.bin
	rm -f *.hex

build:
	rustup default nightly
	rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
	cargo build --target $(TARGET) --$(LEVEL) && arm-none-eabi-readelf -h $(TARGET_FOLDER)/$(TARGET)/$(LEVEL)/$(BINARY)

flash:
	cp $(TARGET_FOLDER)/$(TARGET)/$(LEVEL)/$(BINARY) $(STM_PATH)

doc:
	cargo doc --open

lint:
	cargo clippy