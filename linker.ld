MEMORY {
  kernel (rwx) : ORIGIN = 0x80200000, LENGTH = 128K
}

SECTIONS {
  .text.start : {
    *(.text.start)
  } > kernel

  .text : {
    *(.text)
  } > kernel

  .data : {
    *(.data)
  } > kernel

  .rodata : {
    *(.rodata)
  } > kernel

  .bss : {
    *(.bss)
  } > kernel

  . = ALIGN(16);
  __stack_bottom = .;
  . = . + 16K;
  __stack_top = .;

  . = ALIGN(16);
  __heap_start = .;
  . = . + 1K;
  __heap_end = .;
}
