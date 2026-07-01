# Trits

For a start, let's define what a bit is (this will help build the foundation for trits). As most people will likely know, a bit is the smallest unit of data on a binary platform. A typical binary computer, will think in only 2 states. 0 for false, and 1 for true. This is fairly simple, and we have a lot of research and history in regards to how 0 and 1 can be used for scaling up to practically anything.

We already have precedent for how bits can be used to represent numbers, which in turn can be used to represent characters, strings, and much much more. We have learned to optimize for bits and binary logic in code, algorithms and so on. We have a lot of history with boolean algebra and logic gates as well.

Okay okay, so what even is a trit? We now know that a binary bit is a the smallest unit of data on a binary platform, so a trit must be the same, but for ternary? Yes. In ternary computing, the smallest unit of data is trit. A trit being a 3 state unit, with -1 / N (for negative), 0 / Z (for zero) and (+)1 / P (for positive).

There are a couple of standards for representing trits in a human readable format. With binary we're used to 0 and 1, but with ternary, a precedent hasn't been set yet. Some people use:
```text
Unbalanced ternary:
 0 0.5 1
 0  /  1  (Sometimes can be another character than "/")
 0  1  2

Balanced ternary:
 -  0  +
-1  0  1
 N  Z  P
```

For this project I will only use balanced ternary, this has been chosen for multiple reasons like it's efficiency for signed Words, which also ties into [Word arithmetics](word-arithmetics.md), and also for it fitting well with boolean logic, 0 still maps to False, 1 to True and we can keep -1 as a separate unknown / undefined state.

I personally like to use -1, 0 and 1 for more human readable and mathematical definitions, but I also use N, Z and P when it comes to representations of trits or Words in computing systems. Like for a representation of Word in a terminal, I would use NZP notation, while I'd use -1, 0 and 1 when describing it to a person or working with it more verbally.

So now, we have our definition for trits, also compared to bits:
```d2
direction: down

Bit (binary): {
  0: "0"
  1: "1"
}

Trit (ternary): {
  n: "-1 / N"
  z: "0 / Z"
  p: "(+)1 / P"
}
```

> So why even look into trits as an alternative to bits? If computing has gone well without ternary for so long, then there must be no reason to look into it. Right?

I can understand that sentiment, but I really don't think it has to be so black and white. Yes, we have a lot of history and prior knowledge about binary in so many facets. Yet, there is still many cases looking into trits and ternary logic really makes sense. My personal favorites are in terms of [ternary logic](ternary-logic.md), [Word arithmetics](word-arithmetics.md) and [radix efficiency](word-radix.md).

There is also an argument that ternary could unlock the ability to get more performance in a post moore's law computing world[^1].

[^1]: [USN Ternary Research: Introduction](https://ternaryresearch.com/introduction/)

### How does the computer hold states?

The way a computer reads a 0 or a 1, is based on the voltage (V) that is delivered to the processor. For most common processors and logic gates, 0 will be 0V and 1 will be between 1V and 1,5V. You might be used to 1 being 5V for IC logic gates, but this is not standard for modern CPUs. For ternary, multiple voltage values have been suggested, Huaweis chip for for example uses CNTFETs, compared to CMOS, the standard of binary, and it uses 0V for 0, 1,65V for +1 and 3,3V for -1[^2]. I will hopefully have more to say on this topic [ternary-logic](ternary-logic.md).

[^2]: [Meta-quantum: Huawei has officially unveiled the world’s first ternary logic chip](https://meta-quantum.today/?p=7960) (I don't like this source, but it has the voltage values. I am not proud to use it, so I need to find better research in the future.)

> Why go from 2 to 3 states? Wouldn't it be more effective to go up to 8 or 16, or even 1024 states? That would make it much easier to address a lot of things.

The answer to this comes down to multiple things, like electrical complexity (specifically about voltage stability and thresholds for CMOS[^3]), [radix efficiency](word-radix.md) and wasted space. For a multitude of reasons, we'd prefer to keep it to fewer states, while optimizing for data it can represent.

[^3]: [Scaling, Power, and the Future of CMOS](https://www.engineering.upenn.edu/~leebcc/teachdir/ece299_fall10/Horowitz05_Scaling.pdf)
