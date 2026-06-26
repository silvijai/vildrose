# vildrose

A balanced ternary system *(inspired by the setun project)* attempting to implement a logic and type system *(vildrose-core)*, python bindings *(Snerle)*, a VM *(Hyldeblomst)*, ISA instructions *(Mirabelle)*, an assembler **(Nemunas)*, a debugger *(Anemone)*, a native binary execution *(setern-rv32im)* and an MMIO device integration.

The project also envisions the ability to run a VM with a hybrid CPU, with both ternary and binary based cores. A big part in wanting to do this, is with hopes of progressing research, especially as it pertains to ternary instruction sets and logic in coding.

## Why?

I am super fascinated by ternary as a concept, and want to try and implement it as a structure from the ground up. If all else fails, I'll at least end up understanding computers better

## Docs
(Not yet implemented, should be available through docs branch in the future)

## Roadmap

- [ ] Phase 1: Foundation
  - [ ] Core arithmetic, type and logic implementation
  - [ ] Python bindings (Likely to be pushed back)
- [ ] Phase 2: VM + ISA
  - [ ] Implement minimal ISA
  - [ ] Registers, memory and CPU for VM
  - [ ] Basic execution of hand written instructions
- [ ] Phase 3: Assembler
  - [ ] Lexer
  - [ ] Parser
  - [ ] Debugger
- [ ] Phase 4: MMIO devices
  - [ ] Binary device compatibility
  - [ ] UART
  - [ ] Framebuffer / Display
- [ ] Phase 5: Binary compatibility
  - [ ] RV32I decoder
  - [ ] RV32I executer
  - [ ] ELF loader
  - [ ] Linux syscall forwarding
- [ ] Phase 6: 54 trit extension and RV64I compatibility
- [ ] Phase 7: To be determined
  - [ ] Full system emulation
  - [ ] FPGA soft core synthesis
  - [ ] Ternary native OS
