# Phoenix

🐦‍🔥
A Motorola 68000 CPU emulator.

Work in progress.

## Progress

-   [ ] Implement all instructions
-   [ ] Boot sequence
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
-   [ ] ANDItoCCR
-   [ ] ANDItoSR
-   [ ] CMP
-   [ ] CMPA
-   [ ] CMPI
-   [ ] CMPM
-   [ ] EOR
-   [x] EORI
-   [ ] EORItoCCR
-   [ ] EORItoSR
-   [ ] NEG
-   [ ] NEGX
-   [x] NOT
-   [x] OR
-   [x] ORI
-   [ ] ORItoCCR
-   [ ] ORItoSR

### Control Flow

-   [ ] BCC
-   [x] BRA
-   [x] BSR
-   [x] DBCC
-   [x] HALT
-   [ ] ILLEGAL
-   [x] JMP
-   [x] JSR
-   [ ] NOP
-   [ ] RESET
-   [ ] RTE
-   [ ] RTR
-   [x] RTS
-   [ ] SCC
-   [ ] STOP
-   [ ] TAS
-   [x] TRAP
-   [ ] TRAPV
-   [ ] TST

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
