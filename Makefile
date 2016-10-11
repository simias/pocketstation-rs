# Directory added to the path for rustc to find arm-none-eabi-gcc and
# friends
XCC_PATH := /opt/pksx-tools/bin

# Update PATH to let rustc link the program correctly
export PATH := $(PATH):$(XCC_PATH)

# Path to xargo (use `cargo install xargo` to build and install it)
XARGO := $(HOME)/.cargo/bin/xargo

OBJCPY := $(XCC_PATH)/arm-none-eabi-objcopy
SIZE := $(XCC_PATH)/arm-none-eabi-size

ELF_PATH := target/armv4t-none-eabi/release/pocketstation-rs

.PHONY: all
all: target/exe.bin

target/exe.bin: xargo
	$(SIZE) $(ELF_PATH)
	$(OBJCPY) -O binary $(ELF_PATH) $@

.PHONY: xargo
xargo: 
	$(XARGO) build --release --target armv4t-none-eabi
