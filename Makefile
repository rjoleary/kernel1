AS := /usr/bin/arm-none-eabi-as
LD := /usr/bin/arm-none-eabi-ld
CC := /usr/bin/arm-none-eabi-gcc
RUSTC := rustc --target=arm-unknown-linux-gnueabihf
QEMU := qemu-system-arm

RS_FILES := $(wildcard src/*.rs)
S_FILES := $(patsubst src/%.rs,out/%.s,${RS_FILES})
O_FILES := $(patsubst src/%.rs,out/%.o,${RS_FILES})

.PHONY: build run clean

all: out kernel.elf

out:
	mkdir out

kernel.elf: out/start.o out/syscall.o out/memset.o linker.ld
	${LD} -N -Map out/kernel.map -init start -T linker.ld $^ -o kernel.elf

out/syscall.o: src/syscall.s
	${AS} src/syscall.s -o out/syscall.o

out/start.o: ${RS_FILES}
	${RUSTC} --codegen opt-level=2 --emit obj $< -o $@

out/memset.o: src/memset.c
	${CC} -mfloat-abi=hard -nostdlib -nostartfiles -ffreestanding -c src/memset.c -o out/memset.o

out/%.o: out/%.s
	${RUSTC}

out/%.s: src/%.rs
	${RUSTC} -O --emit asm $< -o $@

# 3G is a magic number because the memory starts at 0x40000000 and the stack at 4G.
run: kernel.elf
	${QEMU} --machine virt --kernel $< --nographic -m 3G

clean:
	rm -rf kernel.elf out/
