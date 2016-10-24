ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* ensure multiboor header is ath the beginning */
        *(.multiboot_header)
    }

    .text :
    {
        *(.text)
    }
}
