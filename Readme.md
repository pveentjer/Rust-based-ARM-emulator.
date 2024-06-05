# Rust based ARM CPU emulator

## About The Project
The primary aim of this project is to provide a possible implementation of modern
processors in software (Rust) for the ARM instruction set.

The goal of the project is to provide some insights in how modern processors potentially
could work.

The goal of the project is not to provide a super fast implementation; for that
code generation using binary translation would be orders of magnitude faster.  

## Warning

This project is toy project. I want to upgrade my Rust skills and I needed a serious
enough challenge to work on. Also, the challenge should initially be without the need 
for concurrency control so that I get a better understanding of ownership. 

### CPU features 

* Pipelined Execution
* Super Scalar Execution
* Out of Order Execution using Tomasulo's algorithm. So only RAW dependencies are preserved.
* Speculative Execution
* Store Buffer
* Performance monitor although not exposed through model specific registers.

### Planned CPU features
* Support for different data types
* One-way fences like LDAR, STLR, LDAPR. 
* Two-way fences like DMB
* Serializing instructions like DSB
* Exclusive access instructions like LDXR, STXR, LDAXR, STLXR
* SMT (aka hyper-threading)
* CMP (aka multicore)
* Working cache (MESI based)
* Write coalescing
* Store buffer out-of-order commit to the cache
* SVE (SIMD)
* NEON (SIMD)

## Supported instructions

### Arithmetic instructions:
* ADD
* SUB
* MUL
* SDIV
* NEG

### Bitwise logical instructions:
* AND
* ORR
* EOR
* MVN

### Memory access instructions:
* LDR
* STR

### Miscellaneous instructions:
* NOP
* MOV
* DSB

### Branch & control instructions:
* B
* BX
* BL
* CBZ
* CBNZ
* CMP
* BEQ
* BNE
* BLE
* BLT
* BGE
* BGT

### Memory barrier instructions:
* DSB (SY)

### Unofficial instructions
* PRINTR: prints the value of a register.

More instructions will be added over time.

## How to run

```bash
cargo run -- --file asm/high_ipc.asm --config cpu.yaml
```

