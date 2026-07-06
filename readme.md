# vildrose

An umbrella project referring to a couple main systems:

- Vildrose-core

A rust based base implementation of ternary logic and types.

- Snerle

bindings for vildrose-core for use in python, C#, C and Zig.

- Hyldeblomst

VM for running ternary and binary ISAs together in an attempt to optimize algorithms and software for ternary and hybrid compute platforms.

- Anemone

A GUI and CLI debugger for Hyldeblomst and Vildlang.

- Vildlang (working title)
  - LSP
  - Fuzzer
  - PKG and build pipeline

Programming language for writing code between binary and ternary. It's main goal is to obfuscate the difference between bits, trits, word sizes and logic, so that that code written targeting both platforms, could be run on both.

Might also become an umbrella term for vildcode, and any language that can translate and to vildcode, and use vildrose bindings. Like a libadalang setup.

- Nemunas

Assembler, lexer, parser and general pipeline for building, producing vildcode (intermediate language code) and interfacing with platform compilation.

Expected pipeline is to build vildcode IL, and then have each platform implement a simple build pipeline, where the vildcode gets translated to native instructions. A path for precompiling for platforms will also be envisioned.

## Why?

I am super fascinated by ternary as a concept, and want to try and implement it as a structure from the ground up. If all else fails, I'll at least end up understanding computers better

## Docs

There are two implementations of docs, one that is [books](https://silvijai.github.io/vildrose/book/), which is about project structure, ternary logic and general info. And the [API-docs](https://silvijai.github.io/vildrose/api/), which are the APIs from the crates.

## Roadmap

- [ ] Phase 1: Foundation
  - [X] Core: arithmetic, type and logic implementation
  - [X] Core: Test suite
  - [ ] Core: API and bindings

- [ ] Phase 2a: VM and ISA
  - [ ] integrate Setnex ISA
    - [ ] 5500FP ISA
    - [ ] REBEL-6 ISA
  - [ ] Build VM pipeline
  - [ ] Expose connections for anemone

// Still unsure whether vildlang will come now, or I will be using libadalang for a start
- [ ] Phase 2b: Vildlang and Nemunas
  - [ ] Define Vildlang syntax and files
  - [ ] Lexer + Parser
  - [ ] LSP
  - [ ] Nemunas: Vildcode definitions
  - [ ] Vildlang to vildcode translation
  - [ ] Vildcode to platform ASM conversion
    - [ ] JIT

- [ ] Phase 3: Implementation groundwork
  - [ ] Linux ABI
  - [ ] Vildlang as a path for translating binary compiled applications to ternary logic (Similar to box64 / wine (Might end up being a separate project))
  - [ ] Binary device compatibility
  - [ ] UART
  - [ ] Framebuffer / Display
  - [ ] USB, WiFi, Bluetooth etc.

- [ ] Phase 4: Beyond
  - [ ] Full linux support
  - [ ] Hybrid CPU stability
  - [ ] Hybrid GPU implementation
  - [ ] FPGA Soft core
  - [ ] Experimenting with ternary native hardware development
