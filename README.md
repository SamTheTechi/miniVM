## Instruction Set

### **General Operations**
| Mnemonic | Opcode      | Description                     |
|----------|-------------|---------------------------------|
| nop      | 0x40000000  | No operation (does nothing).    |
| hlt      | 0x40000010  | Halts the program execution.    |
| int      | 0x40000011  | Triggers an interrupt.          |

### **Data Movement**
| Mnemonic | Opcode      | Description                     |
|----------|-------------|---------------------------------|
| mov      | 0x40000001  | Moves data between registers.   |
| lod      | 0x40000002  | Loads data from memory.         |
| str      | 0x40000003  | Stores data into memory.        |

### **Control Flow**
| Mnemonic | Opcode      | Description                                   |
|----------|-------------|-----------------------------------------------|
| jmp      | 0x40000004  | Unconditional jump to a memory address.       |
| jml      | 0x40000005  | Jump if less than (based on `cmp` result).    |
| jmg      | 0x40000006  | Jump if greater than (based on `cmp` result).|
| jme      | 0x40000007  | Jump if equal (based on `cmp` result).       |
| rtn      | 0x40000008  | Return from a subroutine.                    |

### **Comparison**
| Mnemonic | Opcode      | Description                     |
|----------|-------------|---------------------------------|
| cmp      | 0x40000009  | Compares two values.            |

### **Stack Operations**
| Mnemonic | Opcode      | Description                     |
|----------|-------------|---------------------------------|
| psh      | 0x4000000c  | Pushes a value onto the stack.  |
| pop      | 0x4000000d  | Pops a value from the stack.    |
| pek      | 0x4000000e  | Peeks at the top value on the stack. |
| swp      | 0x4000000f  | Swaps the top two stack values. |

### **Arithmetic Operations**
| Mnemonic | Opcode      | Description                     |
|----------|-------------|---------------------------------|
| add      | 0x40000012  | Adds two values.                |
| sub      | 0x40000013  | Subtracts two values.           |
| mul      | 0x40000014  | Multiplies two values.          |
| div      | 0x40000015  | Divides two values.             |
| mod      | 0x40000016  | Finds the remainder of division.|
| inc      | 0x40000017  | Increments a value by 1.        |
| dec      | 0x40000018  | Decrements a value by 1.        |
| sqt      | 0x40000019  | Computes the square root of a value. |

### **Logical and Bitwise Operations**
| Mnemonic | Opcode      | Description                     |
|----------|-------------|---------------------------------|
| and      | 0x4000001a  | Logical AND operation.          |
| sor      | 0x4000001b  | Logical OR operation.           |
| xor      | 0x4000001c  | Logical XOR operation.          |
| not      | 0x4000001d  | Logical NOT operation.          |
| shl      | 0x4000001e  | Shifts bits left.               |
| shr      | 0x4000001f  | Shifts bits right.              |

### **Input/Output Operations**
| Mnemonic | Opcode      | Description                     |
|----------|-------------|---------------------------------|
| out      | 0x4000000a  | Outputs a value.                |
| sin      | 0x4000000b  | Reads a value from input.       |
