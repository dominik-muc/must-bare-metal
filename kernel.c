
#include "io.h"
#include "types.h"
#include "uart.h"

volatile i32 main() {
    u8 buf[255];
    while (true) {
        uart_putstr((u8*)"> ");
        get_line(buf);
        uart_putstr(buf);
        uart_putstr((u8*)"\r\n");
    }
}
