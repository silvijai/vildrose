# Memory Map

Here protocols for MMIO devices, UART, framebuffer and so on will be defined. This will likely not be as in depth, and might focus on some experimental connections that are more ternary native, as otherwise, a binary core is way more suited for protocols already defined for binary.

In the future, the protocols, memory addresses and so on will be described in depth in this chapter.

## Vildrose core memory map
The allocation of a trit in memory in vildrose core, is currently, using an i8 to represent a trit, and thus, the memory map is as follows:
```d2
direction: right

Byte (1 trit) {
  grid-columns: 8
  grid-gap: 0
  style.fill: transparent

  b7: "unused"
  b6: "unused"
  b5: "unused"
  b4: "unused"
  b3: "unused"
  b2: "unused"
  b1: "trit data" { style.fill: "#2ecc71"; }
  b0: "trit data" { style.fill: "#2ecc71"; }
}
```

This is quite wasteful. More than 75% of the memory currently goes unused, for this a couple of alternatives are being considered, such as packing 3 trits into a single byte, or even 4 trits into a single byte. The memory map for these alternatives is as follows:
```d2
direction: right

Byte (3 trits (tribble)) {
  grid-columns: 8
  grid-gap: 0
  style.fill: transparent

  b7: "unused"
  b6: "unused"
  b5: "trit 3" { style.fill: "#2ecc71"; }
  b4: "trit 3" { style.fill: "#2ecc71"; }
  b3: "trit 2" { style.fill: "#2ecc71"; }
  b2: "trit 2" { style.fill: "#2ecc71"; }
  b1: "trit 1" { style.fill: "#2ecc71"; }
  b0: "trit 1" { style.fill: "#2ecc71"; }
}

Byte (4 trits) {
  grid-columns: 8
  grid-gap: 0
  style.fill: transparent

  b7: "trit 4" { style.fill: "#2ecc71"; }
  b6: "trit 4" { style.fill: "#2ecc71"; }
  b5: "trit 3" { style.fill: "#2ecc71"; }
  b4: "trit 3" { style.fill: "#2ecc71"; }
  b3: "trit 2" { style.fill: "#2ecc71"; }
  b2: "trit 2" { style.fill: "#2ecc71"; }
  b1: "trit 1" { style.fill: "#2ecc71"; }
  b0: "trit 1" { style.fill: "#2ecc71"; }
}
```

Of these two paths, I more so lean towards the tribble approach. As a tribble with 3 trits, quite efficiently scales up into Tryte, Word9, Word27 and other types.

The real issue with packing trits into bytes, is that it makes the memory map more complex, and thus, the CPU will have to do more work to read and write trits from memory. This is a trade off that will have to be considered when deciding on the final memory map.

Tests for speed specifically should be setup, to see what maps best in regards to that.
