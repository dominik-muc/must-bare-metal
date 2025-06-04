#include <stdio.h>
#include "pico/stdlib.h"

#include "uart.h"



void init_pico_stdio() {
    stdio_init_all();
}

void uart_putchar(u8 c) { putchar(c); }

void uart_putstr(u8 str[]) {
    while (*str) {
        uart_putchar(*str);
        str++;
    }
}

u8 uart_getchar() { 
    return (u8)getchar();
}