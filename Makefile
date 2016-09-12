arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

target ?= $(arch)-unknown-linux-gnu
rust_os := target/$(target)/debug/libvvfos_os.a

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))


.PHONY: all clean run iso


all: $(kernel)

clean:
	rm -r build
	rm -r target



rustup.sh:
	curl https://static.rust-lang.org/rustup.sh > rustup.sh
	chmod u+x rustup.sh

rust: rustup.sh
	./rustup.sh --channel=nightly



run: $(iso)
	qemu-system-x86_64 -cdrom $(iso) -s

rerun: clean run

debug: $(iso) rust-os-gdb/bin/rust-gdb
	qemu-system-x86_64 -cdrom $(iso) -s -S

rust-os-gdb/bin/rust-gdb:
	curl -sf https://raw.githubusercontent.com/phil-opp/binutils-gdb/rust-os/build-rust-os-gdb.sh > build-rust-os-gdb.sh
	chmod u+x build-rust-os-gdb.sh
	./build-rust-os-gdb.sh

gdb:
	rust-os-gdb/bin/rust-gdb "build/kernel-x86_64.bin" -ex "target remote :1234"


dirs:
	mkdir -p build/isofiles/boot/grub

iso: $(iso)

$(iso): dirs $(kernel) $(grub_cfg)
	cp $(kernel) build/isofiles/boot/kernel.bin
	cp $(grub_cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles
	rm -r build/isofiles

$(kernel): cargo $(rust_os) $(assembly_object_files) $(linker_script)
	ld -n --gc-sections -T $(linker_script) -o $(kernel) \
		$(assembly_object_files) $(rust_os)

cargo:
	cargo build --target $(target)

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@