.text 0x1000
    # Load values from memory
    lw x1, value1    # Load first number
    lw x2, value2    # Load second number
    add x3, x1, x2   # Add them
    sw x3, result    # Store result

.data 8192
value1: .word 42     # First number
value2: .word 58     # Second number
result: .word 0      # Space for result