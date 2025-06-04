.extern main
.global _start

.section .text
_start:
    la sp, __stack_top  # load address into stack pointer
    call main
    j .                 # infinite loop
