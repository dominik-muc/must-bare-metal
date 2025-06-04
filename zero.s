.extern main
.global _start

.section .text
_start:
    movabsq $__stack_top, %rsp  # load address into stack pointer
    call main
    jmp .                 # infinite loop
