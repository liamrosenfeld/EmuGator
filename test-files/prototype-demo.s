.data
message: .string "test\n"
numbers: .word 1, 2, 3, 4
bytes: .byte 0xFF, 0x42, 0x33
array: .ascii "test"

.text
main:
    # Example I-type instruction example
    ADDI x2, x1, 5      #   x2 = x1 + 5
    XORI x3, x2, 0xFF   #   x3 = x2 ^ 0xFF
    SLLI x1, x2, 2      #   x1 = x2 >> 2

    # U-type instruction example
    LUI x4, 0xFFF       # (load upper immediate) x4 = 0xFFF << 20

    # R-type instruction example
    SUB x6, x3, x2      # x7 = x4 - x2

    # J-type instruction
    JAL x5, function    # jump to function, storing return address in x5
    SW x5, 4(x1)
    XOR x1, x1, x1
    XOR x1, x1, x1
    XOR x1, x1, x1

branch_target:
    ADDI x2, x6, 10
    JALR x5, x5, 0x00 

function:
    # B-type instruction
    BNE x3, x2, branch_target


.data
message: .string "test\n"
numbers: .word 1, 2, 3, 4
bytes: .byte 0xFF, 0x42, 0x33
array: .ascii "test"

.text
main:
    # Example I-type instruction example
    ADDI x2, x1, 5      #   x2 = x1 + 5
    XORI x3, x2, 0xFF   #   x3 = x2 ^ 0xFF
    SLLI x1, x2, 2      #   x1 = x2 >> 2

    # U-type instruction example
    LUI x4, 0xFFF       # (load upper immediate) x4 = 0xFFF << 20

    # R-type instruction example
    SUB x6, x3, x2      # x7 = x4 - x2

    # J-type instruction
    BNE x3, x2, branch_target
    XOR x5, x5, x5
    XOR x5, x5, x5
    # JAL x5, function    # jump to function, storing return address in x5

branch_target:
    ADDI x2, x6, 10
    JALR x5, x5, 0x00 

function:
    # B-type instruction
    BNE x3, x2, branch_target


.data
message: .string "test\n"
numbers: .word 1, 2, 3, 4
bytes: .byte 0xFF, 0x42, 0x33
array: .ascii "test"

.text
main:
    # Example I-type instruction example
    ADDI x2, x1, 5      #   x2 = x1 + 5
    XORI x3, x2, 0xFF   #   x3 = x2 ^ 0xFF
    SLLI x1, x2, 2      #   x1 = x2 >> 2

    # U-type instruction example
    LUI x4, 0xFFF       # (load upper immediate) x4 = 0xFFF << 20

    # R-type instruction example
    SUB x6, x3, x2      # x7 = x4 - x2

    # J-type instruction
    JAL x5, function    # jump to function, storing return address in x5
    XOR x5, x5, x5
    XOR x1, x1, x1
    XOR x1, x1, x1
    XOR x1, x1, x1
    XOR x1, x1, x1
    XOR x1, x1, x1

function:
    # B-type instruction
    BNE x3, x2, branch_target
    XOR x1, x1, x1
    XOR x1, x1, x1
    #XOR x6, x5, x5 #THIS EXECUTES
    # CAN'T DO BACKWARDS BRANCHING
    # HAVING TO SUBTRACT 8

branch_target:
    ADDI x2, x6, 10
    JALR x1, x5, 0x00 
    XOR x1, x1, x1