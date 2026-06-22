# not-etherium

A minimal **Ethereum Virtual Machine (EVM)** written from scratch in Rust — no external
crates, no `revm`, no shortcuts. Built to understand how the EVM actually executes
bytecode: a 256-bit stack machine with its own memory, persistent storage, and calldata.

> Learning / portfolio project. The goal is a functional EVM core that can run real
> Ethereum bytecode.

## How it works

The EVM is a big-endian, 256-bit **stack machine**. Everything is a `[u8; 32]` word.

```
bytecode ──► execute() ──► loop:
                             read opcode at pc
                             advance pc
                             match opcode -> mutate stack / memory / storage
                           ──► returns Vec<u8> (RETURN data) or error
```

State lives in the `VM` struct (`src/vm.rs`):

| Field      | Type                        | What it is                          |
|------------|-----------------------------|-------------------------------------|
| `stack`    | `Vec<[u8; 32]>`             | the 256-bit operand stack           |
| `pc`       | `u64`                       | program counter                     |
| `gas`      | `u64`                       | gas counter (not metered yet)       |
| `memory`   | `Vec<u8>`                   | byte-addressable scratch memory     |
| `calldata` | `Vec<u8>`                   | input data for the call             |
| `storage`  | `HashMap<[u8;32], [u8;32]>` | persistent key/value storage        |

256-bit arithmetic is hand-rolled (`add_u256`, `sub_u256`, `mul_u256`, `divmod_u256`) since
Rust has no native u256 — see the bottom of `src/vm.rs`. Signed ops build on these:
`is_neg` tests the top bit, `neg_u256` does two's-complement negation (`NOT + 1`), and
`sdiv`/`smod` strip signs, divide magnitudes via `divmod_u256`, then re-apply the sign.

## Implemented opcodes

| Group       | Opcodes                                             |
|-------------|-----------------------------------------------------|
| Arithmetic  | `ADD` `MUL` `SUB` `DIV` `SDIV` `SMOD`                |
| Comparison  | `LT` `GT` `EQ` `ISZERO` `SLT` `SGT`                  |
| Bitwise     | `AND` `OR` `XOR` `NOT`                               |
| Memory      | `MLOAD` `MSTORE` `MSTORE8`                           |
| Storage     | `SLOAD` `SSTORE`                                     |
| Control flow| `JUMP` `JUMPI` `JUMPDEST`                            |
| Stack       | `PUSH1`–`PUSH32` `POP` `DUP1`–`DUP16` `SWAP1`–`SWAP16` |
| Calldata    | `CALLDATASIZE` `CALLDATALOAD`                        |
| Halt        | `STOP` `RETURN`                                      |

## Running it

```bash
cargo run
```

`main.rs` runs a sample program:

```
PUSH2 0x0102    # push the value 0x0102
PUSH1 0x00      # offset 0
MSTORE          # store it at memory[0..32]
PUSH1 0x20      # size 32
PUSH1 0x00      # offset 0
RETURN          # return memory[0..32]
```

Expected output: the 32-byte word ending in `...0102`.

## Tests

```bash
cargo test
```

Unit tests live in `src/vm.rs` and exercise the signed ops directly — covering the
sign-rule edge cases (negate-on-different-signs for `SDIV`, sign-follows-dividend for
`SMOD`, the raw-bytes trap for `SLT`) plus division by zero and the `divmod_u256` split.

## Roadmap

- [ ] Gas metering (per-opcode cost, out-of-gas halt)
- [ ] Finish signed ops (`SAR`, `SIGNEXTEND`)
- [ ] `MOD` `ADDMOD` `MULMOD` `EXP`
- [ ] Shifts (`SHL` `SHR` `SAR`)
- [ ] `KECCAK256`
- [ ] Environment opcodes (`CALLER`, `CALLVALUE`, `ADDRESS`, `PC`, `MSIZE`, ...)
- [ ] `LOG0`–`LOG4`
- [ ] Inter-contract calls (`CALL`, `CREATE`)

## Layout

```
src/
  main.rs   # entry point + sample bytecode
  vm.rs     # the VM: execute loop + u256 helpers
```
