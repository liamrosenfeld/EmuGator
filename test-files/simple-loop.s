# Simple program to sum numbers 1 to 5
# Result (15) will be stored in memory

.data
result: .word 0    # Storage for result

.text
main:
    # Initialize registers
    ADDI x5, x0, 0      # sum = 0
    ADDI x6, x0, 1      # i = 1
    ADDI x7, x0, 5      # end = 5

loop:
    ADD x5, x5, x6      # sum += i
    ADDI x6, x6, 1      # i++
    SLT x8, x7, x6      # if i > 5 then x8 = 1
    BEQ x8, x0, loop    # if x8 == 0 then loop

    # Store result
    LUI x9, 0           # Initialize high bits of address to 0
    ADDI x9, x9, 0      # Add offset to get address of result
    SW x5, 0(x9)        # Store sum at result

    # End program
    ECALL