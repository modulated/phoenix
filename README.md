# Phoenix

🐦‍🔥
A Motorola 68000 CPU emulator.

Work in progress.

## Progress

-   [ ] Implement all instructions
-   [ ] Boot sequence
-   [ ] Memory map
-   [ ] Display output
-   [ ] Complex program
-   [ ] Extensible API
-   [ ] Test runner

## Instructions

### Moves

-   [x] CLR
-   [x] EXG
-   [x] LEA
-   [x] MOVE
-   [x] MOVEA
-   [x] MOVEtoCCR
-   [x] MOVEfromSR
-   [x] MOVEtoSR
-   [x] MOVE USP
-   [ ] MOVEM
-   [ ] MOVEP
-   [x] MOVEQ
-   [x] SWAP

### Arithmetic

-   [x] ADD
-   [ ] ADDA
-   [x] ADDI
-   [ ] ADDQ
-   [ ] ADDX
-   [ ] DIVS
-   [ ] DIVU
-   [ ] MULS
-   [ ] MULU
-   [ ] SUB
-   [ ] SUBA
-   [ ] SUBI
-   [ ] SUBQ
-   [ ] SUBX

### Logic

-   [x] AND
-   [ ] ANDI
-   [x] ANDItoCCR
-   [x] ANDItoSR
-   [x] CMP
-   [x] CMPA
-   [ ] CMPI
-   [ ] CMPM
-   [ ] EOR
-   [x] EORI
-   [x] EORItoCCR
-   [x] EORItoSR
-   [x] NEG
-   [ ] NEGX
-   [x] NOT
-   [x] OR
-   [x] ORI
-   [x] ORItoCCR
-   [x] ORItoSR

### Control Flow

-   [ ] BCC
-   [x] BRA
-   [x] BSR
-   [x] DBCC
-   [x] HALT
-   [x] ILLEGAL
-   [x] JMP
-   [x] JSR
-   [x] NOP
-   [ ] RESET
-   [ ] RTE
-   [ ] RTR
-   [x] RTS
-   [ ] SCC
-   [ ] STOP
-   [ ] TAS
-   [x] TRAP
-   [x] TRAPV
-   [x] TST

### Stack

-   [x] CHK
-   [x] LINK
-   [x] PEA
-   [x] UNLK

### Bitwise Operations

-   [ ] ASL
-   [ ] ASR
-   [ ] BCHG
-   [ ] BCLR
-   [ ] BSET
-   [ ] BTST
-   [x] EXT
-   [ ] LSL
-   [ ] LSR
-   [ ] ROL
-   [ ] ROR
-   [ ] ROXL
-   [ ] ROXR

### BCD

-   [ ] ABCD
-   [ ] NBCD
-   [ ] SBCD
