## Table of Contents
- [Instructions](#instructions)
  - [Data Movement Instructions](#data-movement-instructions)
  - [Control Flow Instructions](#control-flow-instructions)
  - [Stack Instructions](#stack-instructions)
  - [Arithmetic Instructions](#arithmetic-instructions)
  - [Logical Instructions](#logical-instructions)
  - [I/O Instructions](#io-instructions)
- [Registers](#registers)

---

## Instructions

### Data Movement 
| Instruction | Opcode | Parameters | Description                      |
|-------------|--------|------------|----------------------------------|
|  `mov`      | `0x01` |   2        | Insert value into register       |
|  `lod`      | `0x02` |   2        | Load value into register         |
|  `str`      | `0x03` |   2        | Store value from register        |
|  `clr`      | `0x0a` |   1        | Clear register                   |
|  `swp`      | `0x0d` |   2        | Swap values between registers    |

### Control Flow Instructions
| Instruction | Opcode | Parameters | Description                      |
|-------------|--------|------------|----------------------------------|
|  `nop`      | `0x00` |   0        | No operation                    |
|  `jmp`      | `0x04` |   1        | Unconditional jump              |
|  `jml`      | `0x05` |   1        | Jump if less than               |
|  `jmg`      | `0x06` |   1        | Jump if greater than            |
|  `jme`      | `0x07` |   1        | Jump if equal                   |
|  `jmz`      | `0x08` |   1        | Jump if zero                    |
|  `cmp`      | `0x09` |   2        | Compare values                  |
|  `cal`      | `0x0b` |   1        | Call subroutine                 |
|  `ret`      | `0x0c` |   0        | Return from subroutine          |
|  `hlt`      | `0x14` |   0        | Halt execution                  |

### Stack Instructions
| Instruction | Opcode | Parameters | Description                      |
|-------------|--------|------------|----------------------------------|
|  `psh`      | `0x11` |   1        | Push to stack                   |
|  `pop`      | `0x12` |   1        | Pop from stack                  |
|  `pek`      | `0x13` |   1        | Peek at top of stack            |

### Arithmetic Instructions
| Instruction | Opcode | Parameters | Description                      |
|-------------|--------|------------|----------------------------------|
|  `add`      | `0x15` |   2        | Addition                        |
|  `sub`      | `0x16` |   2        | Subtraction                     |
|  `mul`      | `0x17` |   2        | Multiplication                  |
|  `div`      | `0x18` |   2        | Division                        |
|  `mod`      | `0x19` |   2        | Modulus                         |
|  `inc`      | `0x1a` |   1        | Increment                       |
|  `dec`      | `0x1b` |   1        | Decrement                       |
|  `sqt`      | `0x1c` |   1        | Square root                     |

### Logical Instructions
| Instruction | Opcode | Parameters | Description                      |
|-------------|--------|------------|----------------------------------|
|  `and`      | `0x1d` |   2        | Logical AND                     |
|  `sor`      | `0x1e` |   2        | Logical OR                      |
|  `xor`      | `0x1f` |   2        | Logical XOR                     |
|  `not`      | `0x20` |   1        | Logical NOT                     |
|  `shl`      | `0x21` |   2        | Shift left                      |
|  `shr`      | `0x22` |   2        | Shift right                     |

### I/O Instructions
| Instruction | Opcode | Parameters | Description                      |
|-------------|--------|------------|----------------------------------|
|  `out`      | `0x0e` |   1        | Output value                    |
|  `sin`      | `0x0f` |   1        | Input value                     |
|  `nli`      | `0x10` |   0        | New line output                 |

---

## Registers

|  Register  |  Opcode  |
|------------|----------|
|    `r0`    |  `0xa5`  |
|    `r1`    |  `0xa6`  |
|    `r2`    |  `0xa7`  |
|    `r3`    |  `0xa8`  |
|    `r4`    |  `0xa9`  |
|    `r5`    |  `0xaa`  |
|    `r6`    |  `0xab`  |
|    `r7`    |  `0xac`  |
|    `rf`    |  `0xad`  |
|    `rz`    |  `0xae`  |



---
