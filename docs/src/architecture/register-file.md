# Register File

The register file uses 27 registers, with x0 reserved as a hardwired zero.

This fits the ternary design nicely because 27 is 3^3.

A couple of decisions will also have to be made about whether the register list should be larger, and accommodate some postbox registers that can be shared between the ternary cores and binary cores.
