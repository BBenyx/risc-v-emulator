/*
   0:   00700293                li      t0,7
   4:   00500313                li      t1,5
   8:   00000393                li      t2,0
   c:   00030863                beqz    t1,1c <done>
  10:   005383b3                add     t2,t2,t0
  14:   fff30313                addi    t1,t1,-1
  18:   ff5ff06f                j       c <loop>
*/
// NOTE: code starts at address 0, so 0x0C is the address of "loop"

.section .text
.globl _start

_start:
    addi t0, x0, 7      // t0 = number to multiply
    addi t1, x0, 5      // t1 = loop counter (5 iterations)
    addi t2, x0, 0      // t2 = result accumulator

loop:
    beq  t1, x0, done   // if counter == 0, exit
    add  t2, t2, t0     // result += number
    addi t1, t1, -1     // counter--
    jal  x0, loop       // jump to loop

done:
    // t2 should now hold 7 * 5
