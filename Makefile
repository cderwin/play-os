arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/playos-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_src := $(wildcard src/arch/$(arch)/*.asm)
assembly_obj := $(patsubst src/arch/$(arch)/%.asm, build/arch/$(arch)/%.o, $(assembly_src))

target ?= $(arch)-unknown-linux-gnu
rust_os := target/$(target)/debug/libplay_os.a

.PHONY := all clean run iso

all: $(kernel)

clean:
	@rm -r build

run: $(iso)
	qemu-system-$(arch) -cdrom $(iso)

iso: $(iso)

isofiles := build/arch/$(arch)/isofiles
$(iso): $(kernel) $(grub_cfg)
	@mkdir -p $(isofiles)/boot/grub
	@cp $(kernel) $(isofiles)/boot/kernel.bin
	@cp $(grub_cfg) $(isofiles)/boot/grub/grub.cfg
	grub-mkrescue /usr/lib/grub/i386-pc -o $@ $(isofiles)

$(kernel): cargo $(rust_os) $(assembly_obj) $(linker_script)
	ld -n -T $(linker_script) -o $@ \
		$(assembly_obj) $(rust_os)

cargo:
	cargo build --target $(target)

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@
