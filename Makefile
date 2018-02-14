.PHONY: build run clean

build: main

main: src/main.s
	/usr/bin/arm-none-eabi-as src/main.s -o main

src/main.s: src/main.rs
	rustc --target=arm-unknown-linux-gnueabihf -O --emit asm src/main.rs -o src/main.s

run: main
	qemu-system-arm --machine virt --kernel main --nographic

clean:
	rm -f main.s main
