AS := /usr/bin/arm-none-eabi-as
RUSTC := rustc --target=arm-unknown-linux-gnueabihf
QEMU := qemu-system-arm

RS_FILES := $(wildcard src/*.rs)
S_FILES := src/start.s

.PHONY: build run clean

all: kernel.elf

kernel.elf: ${S_FILES}
	${AS} $^ -o $@

src/%.s: src/%.rs
	${RUSTC} -O --emit asm $< -o $@

run: kernel.elf
	${QEMU} --machine virt --kernel $< --nographic

clean:
	rm -f src/*.s kernel.elf
