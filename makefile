### RUST
CC = cargo
LINK_SCRIPT = kernel.ld
RUST_FLAGS = -C link-arg=-T$(LINK_SCRIPT) -C linker=rust-lld
RUST_TARGET = riscv64gc-unknown-none-elf
BIN_NAME = kernel
KERNEL = kernel.elf

### QEMU
QEMU = qemu-system-riscv64
QEMU_MACHINE = -machine virt -cpu rv64 -bios default
QEMU_HW = -smp 1 -m 128M
QEMU_IO = -nographic -serial mon:stdio --no-reboot
QEMU_DEBUG = -d cpu_reset,unimp,guest_errors,int -D qemu.log
QEMU_KERNEL = -kernel $(KERNEL)
QEMU_FLAGS = $(QEMU_MACHINE) $(QEMU_HW) $(QEMU_IO) $(QEMU_DEBUG) $(QEMU_KERNEL)

.PHONY: run build exec check clean

build:
	@echo "Building kernel..."
	RUSTFLAGS="$(RUST_FLAGS)" $(CC) build --bin $(BIN_NAME) --target $(RUST_TARGET)
	cp target/$(RUST_TARGET)/debug/$(BIN_NAME) $(KERNEL)


exec:
	@echo "Running QEMU..."
	$(QEMU) $(QEMU_FLAGS)

run: build exec

check:
	$(CC) check --target $(RUST_TARGET)

clean:
	$(CC) clean
	rm -f $(KERNEL)
