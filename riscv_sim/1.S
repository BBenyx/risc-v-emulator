/*
0:   00300793                li      a5,3
4:   02f787bb                mulw    a5,a5,a5
*/

.section .text
.globl _start

_start:
    li a5,3        # a5 = register, 3 = immediate number
    mulw a5,a5,a5

