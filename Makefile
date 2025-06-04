CROSS_COMPILE=riscv64-elf-

all:
	${CROSS_COMPILE}as -o zero.o -c zero.s
	${CROSS_COMPILE}gcc -T linker.ld *.c zero.o -static -nostdlib -ffreestanding -mcmodel=medany -o kernel.elf
	${CROSS_COMPILE}objcopy kernel.elf -O binary kernel.bin

clean:
	rm -rf *.o
	rm -rf *.bin
	rm -rf *.elf

run:
	qemu-system-riscv64 -kernel kernel.bin -machine virt
