# ISA

The ISA is fixed-format and RISC-like, but includes ternary-native instructions.

A lot of decisions still need to be made around how the ISA is implemented, to keep it inline with how a proper RISC-V ternary implementation would look. For this part, I'll need to read into how RV32IM (the M just signifies multiplication and division) is set up, likely as part of my attempt to integrate a binary core into here as well.

A couple instructions that would be unique to a ternary system would be a TSIGN instruction, TCON for consensus and some 3 way branches.
