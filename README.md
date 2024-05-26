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
-   [ ] EXG
-   [x] LEA
-   [x] MOVE
-   [ ] MOVEA
-   [ ] MOVEtoCCR
-   [x] MOVEfromSR
-   [ ] MOVEtoSR
-   [ ] MOVE USP
-   [ ] MOVEM
-   [ ] MOVEP
-   [x] MOVEQ
-   [ ] SWAP

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

-   [ ] AND
-   [ ] ANDI
-   [ ] ANDItoCCR
-   [ ] ANDItoSR
-   [ ] CMP
-   [ ] CMPA
-   [ ] CMPI
-   [ ] CMPM
-   [ ] EOR
-   [ ] EORI
-   [ ] EORItoCCR
-   [ ] EORItoSR
-   [ ] NEG
-   [ ] NEGX
-   [ ] NOT
-   [ ] OR
-   [x] ORI
-   [ ] ORItoCCR
-   [ ] ORItoSR

### Control Flow

-   [ ] BCC
-   [x] BRA
-   [x] BSR
-   [ ] DBCC
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
-   [ ] TRAP
-   [ ] TRAPV
-   [ ] TST

### Stack

-   [ ] CHK
-   [x] LINK
-   [ ] PEA
-   [ ] UNLK

### Bitwise Operations

-   [ ] ASL
-   [ ] ASR
-   [ ] BCHG
-   [ ] BCLR
-   [ ] BSET
-   [ ] BTST
-   [ ] EXT
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
