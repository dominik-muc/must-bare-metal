#include "io.h"
#include "uart.h"

void get_line(u8 buf[]) {
    u8 i = 0;
    while (i < 255) {
        u8 c = uart_getchar();
        switch (c) {
        case '\r':
            buf[i] = 0;
            uart_putchar('\r');
            uart_putchar('\n');
            return;
        default:
            buf[i++] = c;
            uart_putchar(c);
        }
    }
}
