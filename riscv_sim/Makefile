all: 1.bin 2.bin 3.bin

%.elf: %.S
	riscv64-unknown-elf-gcc -march=rv64g -nostdlib -nostartfiles -Ttext=0x0 $< -o $@

%.bin: %.elf
	riscv64-unknown-elf-objcopy -O binary $< $@
