# Architecture Overview

The main things on the horizon for the CPU architecture are the ISA (both a ternary native one, and a hybrid one for controlling a ternary core and a binary code based on RV32I), memory, registers and words as native sizing.

In general a lot of thoughts have been made in regards to how the CPU should be handled. Should it only have ternary cores? If it is a hybrid system, how do they communicate? Can they perform each others tasks, when they themselves aren't otherwise in use? A lot of this is what the project aims to answer, and later likely tie it into a research project.
