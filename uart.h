#pragma once
#include "types.h"


void init_pico_stdio();

void uart_putchar(u8 c);

void uart_putstr(u8 str[]);

u8 uart_getchar();
