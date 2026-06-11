/*
   0:   ff010113                addi    sp,sp,-16
   4:   00700293                li      t0,7
   8:   00512023                sw      t0,0(sp)
   c:   00c000ef                jal     18 <double_value>
  10:   00012303                lw      t1,0(sp)
  14:   0240006f                j       38 <done>
  18:   ff010113                addi    sp,sp,-16
  1c:   00112623                sw      ra,12(sp)
  20:   01012283                lw      t0,16(sp)
  24:   005282b3                add     t0,t0,t0
  28:   00512823                sw      t0,16(sp)
  2c:   00c12083                lw      ra,12(sp)
  30:   01010113                addi    sp,sp,16
  34:   00008067                ret
*/
// NOTE: for the emulator, set sp (stack pointer) to whatever you need

.section .text
.globl _start

_start:
    addi sp, sp, -16    // allocate 16 bytes
    addi t0, x0, 7
    sw   t0, 0(sp)      // store argument at sp

    // ra = return address, typically used for calling a function
    jal  ra, double_value

    // After return, result is at 0(sp)
    lw   t1, 0(sp)      // t1 now contains 14 (7 * 2)

    jal  x0, done

//-----------------------------------------------------------
// FUNCTION: double_value
// input: argument at sp (caller)
// output: result stored back to same location
//-----------------------------------------------------------

double_value:
    addi sp, sp, -16
    sw   ra, 12(sp)     // save return address

    // Load argument (which is 16 bytes above this frame)
    lw   t0, 16(sp)

    add  t0, t0, t0     // t0 = t0 * 2

    // Store result back where caller expects it
    sw   t0, 16(sp)

    // Restore and return
    lw   ra, 12(sp)
    addi sp, sp, 16
    jalr x0, 0(ra)

done:
    
