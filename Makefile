##
# Make utility to automate boring and repetitive commands
#
# @file Makefile
# @version 0.1

all: build

BINARY=stm
TARGET=thumbv7em-none-eabihf
TARGET_FOLDER=target
LEVEL=release
STM_PATH=/run/media/luca/NODE_L432KC

EL=arm-none-eabi-readelf
CP=arm-none-eabi-objcopy
OD=arm-none-eabi-objdump
SE=arm-none-eabi-size

.PHONY: clean build flash doc lint gdb openocd help

clean:			## Clean all artifacts
	cargo clean && rm -f $(TARGET_FOLDER)
	rm -f *.bin
	rm -f *.hex

build:			## Build project
	rustup default stable
	rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
	cargo build --target $(TARGET) --$(LEVEL) && arm-none-eabi-readelf -h $(TARGET_FOLDER)/$(TARGET)/$(LEVEL)/$(BINARY)

flash: build	## Flash board memory
	arm-none-eabi-objcopy -O binary $(TARGET_FOLDER)/$(TARGET)/$(LEVEL)/$(BINARY) $(BINARY).bin
	cp $(BINARY).bin $(STM_PATH)

doc:			## Open documentation
	cargo doc --open

lint:			## Runt linter
	cargo clippy

gdb:			## Open gdb server
	gdb -q -x openocd.gdb $(TARGET_FOLDER)/$(TARGET)/$(LEVEL)/$(BINARY)

openocd:		## Attach to an opened gdb session
	openocd -f interface/stlink-v2-1.cfg -f nucleo_l432kc.cfg

help:
	@echo "Usage: make [command]"
	@echo "Make utility to automate boring and repetitive commands."
	@echo
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo
