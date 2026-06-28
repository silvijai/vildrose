# Ternary Primer

Balanced ternary uses three digit values:

- N = -1
- Z = 0
- P = +1

There is also concepts of systems with unbalanced ternary that use 0, 1 and 2. This is also a fair choice, but for this project we will use balanced ternary, as it has benefits in natural system for negative numbers, and clean arithmetics for a lot of things. In the future some implementations might be made for an unsigned word, but for now, it will stay with using balanced ternary and signed words.

A lot of other terms are also worth defining for this project. I will try and compare them with binary logic, to make it more generally approachable.

## Kleenes
Kleenes are ternaries answer to booleans. A boolean, or bool for short, has two states, False (0) and True (1). This is what we're used to with logic, an if statement for example will say "if x == y", which can be understand it two ways. Either x and y are equal, returning True. Or they're different, and it returns False (0).

But what about other cases? What if the types don't match? What if the logic isn't directly binary? One example of this is a state like "is x true?", it seems simple, but what if x is undefined, or only going to be defined in the future? Natural instinct is to return False, as it can't be True, but actually, it can't be False either before it's defined.

In this case, a Kleene would fit well. It describes three states:
- False (0)
- True (1)
- Unknown (-1)

In the specific case of x being undefined, or it's definition / implementation is in the future, you'd likely want to throw an exception or error in proper binary programming. But that shouldn't detract from the cases where Kleenes still can offer a benefit.

And their benefits are well known for a lot of user input related fields. Many languages have a notion of Undefined, as an extra state added to Booleans. Example include SQL, JS, and certain forms of config files. But even within logic, there are cases where something is neither True, nor False. Generally any form of answer check is quite fitting for Kleenes, "did this test pass or fail?", it hasn't run yet, so we can mark it's state as Unknown (-1).

Kleenes also have interesting truth tables for logic gates, I could generally recommend reading both the Wikipedia article for [Three-valued logic](https://en.wikipedia.org/wiki/Three-valued_logic) and [An Introduction to Many-Valued and Fuzzy Logic](https://www.cambridge.org/core/books/an-introduction-to-manyvalued-and-fuzzy-logic/4E9F5DF9CFC62C4FE67C052F559A141B). With the unknown state especially unlocking many patterns unseen in binary logic.

## Storage (TODO: Finish this)
Log3n ternary storage efficiency. Ternary holding more values for a similar amount of data points, Word27 having between -3,812,798,742,493 and 3,812,798,742,493 values. While 32 bits only has between -2,147,483,647 and 2,147,483,647.

https://www.manitlab.org/blog/posts/tritfs-address-space/

## Words
A Word refers to the collection of bits / trits in an array like manner. A Byte is a form of Word, specifically a binary Word of 8 bits. With this, an int8 can also be described as a binary Word8. However, when we use Word in this project, it refers to a ternary Word of trits that are devisable by 3. The main forms are:
- Word9 (also called Tryte, similar to a Byte / i8)
- Word27 (similar to i32)
- Word54 (similar to i64)
- Word108 (similar to i128)

Words will be used in a similar way as ints in this project, and Tryte specifically, will be the ternary answer to Bytes.

With this defined, we can define a system architecture like we're used to with Binary. My computer for example has a 64-bit CPU and 16 gigabytes of RAM. A ternary computer in a similar range would have a 54-trit CPU and 18 gigatrytes of RAM. (note that I don't know if I did the conversion that well, and that ternary memory would hold more storage due to having one more state for using trits over bits, and a tryte having one more trit, than bytes have bits)

Ternary Words also have a couple of interesting benefits, my favorite relating to it's signed nature by default. How do you know whether a ternary Word is positive or negative? Look through it's trits, the first non zero trit from left to right will correspond to it's sign, and if it's 0, then that perfectly fits with the 0 value of a trit as well. Want to flip the sign of a Word? Easy, just flip the sign of each trit in the word. Want the absolute value of the Word? Just flip all the negative trits. Also the fact that it's max and min value are the same, just with one positive, the other negative is also deliciously simple.

If you find this type of manipulation of Words as interesting as I do, then I can strongly recommend playing with a [balanced ternary calculator](https://countingmethods.com/balanced-ternary-converter-tool/).

## Theoretical advantage
We've already seen that trits can be used to pack more data, we've seen it's better for some logic, and even some of it's arithmetic is really clean, but what does it matter if it can't actually have an advantage in practice. Well actually, I do believe that it (could) have an advantage in practice, and that's what I hope to prove with this project.

Right now we know of some parts that are more efficient, theoretically some code snippets could be optimized with using better branching features, letting you branch on more cases with fewer instructions than in binary, AI inference and machine learning optimizations in general (example, [BitNet-b1.58](https://github.com/microsoft/BitNet)). I would argue for certain math arithmetics as well, but in practice, it's unlikely to be faster, and that's due to the big reason ternary is going to have an uphill battle...

*Binary logic already has a huge ecosystem*. Practically all of our software is written for binary hardware, binary logic, binary I/O and way more. Until new systems are created and supported, ternary is unlikely to get a huge foothold. Binary will keep its advantage in I/O that interfaces with devices and definitions that are binary native, like UART, USB, displays and way more. Not to mention, binary processors and other hardware has been manufactured and perfected for decades. CMOS is a huge industry within logic gates alone, one that cannot be translated to ternary reasonable, and instead requires new technologies.

How long will it take to have a physical ternary CPU, that can even just somewhat rival what we have now with binary? likely, not in the next few years at least. A lot of research has been made in this field, but to get the processor node size down, making it competitive with binary, and getting software support are all huge undertakings. And it leads to a chicken and the egg question, what needs to come first? The hardware, or the software? I think it's clear that the answer is both, and I'm better at software than hardware, so that's where I put my effort.

I think ternary systems will have similar adoption struggles as RISC-V and ARM. By now ARM and AARCH is really well adopted. Devices running it are quite efficient, run well and often give a better experience for the user. For the longest time it was relegated to mobile devices, but since then, they've gathered a major foothold in the laptop and server markets as well. RISC-V is still in the early phases in adoption, it has some hardware boards, and software support is growing, but it's still far from a practical day to day platform.

So what is my point with this? I think ternary will be a slow process to get into being a product people will want to use, BUT, I don't think it's impossible. My personal belief is that ternary will be like ARM, at first a bit of a niche with some proven benefits, but until it can stand toe to toe with it's competitors like x86, it might as well just be a research technology. I think (and hope) that ternary will be proven and grow through smaller efforts, both in research, and specific cases like AI and efficiency. In my ideal world, it would take a while, but it would become more and more proven year by year, until before you know it, it actually has planted itself into certain workflows and positions solidly.

TL;DR: Ternary is really cool and interesting
