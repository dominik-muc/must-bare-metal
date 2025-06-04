ENTRY(_start)

MEMORY {
  rom (rwx) : ORIGIN = 0x80200000, LENGTH = 1M
  ram (rw) : ORIGIN = 0x80300000, LENGTH = 1M
}

SECTIONS {
    .text : {
        *zero.o(.text*)
        *(.text*)
    } > rom

    .data : {
        *(.text*)
    } > ram

    . = ALIGN(16);
    __stack_top = . + 4K;
    __stack_bottom = .;
}
