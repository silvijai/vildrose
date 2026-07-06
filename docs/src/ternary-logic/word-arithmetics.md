# Arithmetics

For a start before going into specific cases of how arithmetics are performed on words in vildrose-cores code, I would like to make sure there's a blanket understanding for binary and ternary arithmetics and how it relates to the decimal (base 10) we're used to.

As an absolute baseline, it is important to understand what a base is, for this, we will start with an example in decimal. When counting in decimal, we go from 0 to 9, and once we reach 9, we loop over to 10. We can explain this as the one being in the 10s place. One way to understand this, is by representing it like this:

```d2
direction: right

Decimal counting {
  grid-columns: 4
  grid-gap: 0
  style.fill: transparent

  d0: "i * 10^0 (= 1)"
  d1: "i * 10^1"
  d2: "..."
  d3: "i * 10^n"
}
```

Here *i* is representing index / input and can be any natural number between 0 and 9 (0, 1, 2, 3, 4, 5, 6, 7, 8 or 9).

This might seem quite verbose compared to how we typically think about the decimal counting system, but this really is the most basic representation of it.

With this any decimal number can be deconstructed and understood. 25 for example, can be understand as 2 tens, and 5 ones.

But other counting systems also exist. You might already be familiar with binary (base 2) or hexadecimal (base 16):

```d2
direction: down

Binary counting {
  grid-columns: 4
  grid-gap: 0
  style.fill: transparent

  d0: "i * 2^0 (= 1)"
  d1: "i * 2^1"
  d2: "..."
  d3: "i * 2^n"
}

Hexadecimal counting {
  grid-columns: 4
  grid-gap: 0
  style.fill: transparent

  d0: "i * 16^0 (= 1)"
  d1: "i * 16^1"
  d2: "..."
  d3: "i * 16^n"
}
```

Here *i* for binary can only be 0 or 1. And for hexadecimal *i* has to be between 0 and F (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A (10), B (11), C (12), D (13), E (14), F (15)).

With this you might see the pattern. The expression for counting can be expressed as such:

i * base^n = result

So for ternary, we can set up the same type of diagram as for binary:

```d2
Ternary counting {
  grid-columns: 4
  grid-gap: 0
  style.fill: transparent

  d0: "i * 3^0 (= 1)"
  d1: "i * 3^1"
  d2: "..."
  d3: "i * 3^n"
}
```

In ternary, *i* can either be represented by 0, 1 or 2 for unbalanced ternary, or by using -1, 0 or 1 for balanced ternary (though I will use N, Z or P, since it better aligns to the code). Because of the benefits to arithmetics of balanced ternary, that is gonna be used for the examples here. Unbalanced ternary might not even be implemented, and instead only managed by software layers later on.

So let's go through some examples of numbers in each of these base systems:
| Decimal | Binary | Hexadecimal | Balanced Ternary | Unbalanced Ternary |
| ------- | ------ | ----------- | ---------------- | ------------------ |
| 0       | 0      | 0           | Z                | 0                  |
| 1       | 1      | 1           | P                | 1                  |
| 2       | 10     | 2           | PN               | 2                  |
| 3       | 11     | 3           | PZ               | 10                 |
| 5       | 101    | 5           | PNP              | 12                 |
| 8       | 1000   | 8           | PNN              | 22                 |
| 13      | 1101   | D           | PPP              | 111                |
| 25      | 11001  | 19          | PZNP             | 221                |

At this point another pattern might appear, the smaller the base, the more digits are needed to represent a number. Yet we still don't use hexadecimal in hardware, both because of complexity, but also because of [radix efficiency](word-radix.md).

But this leads to another question, what about negative numbers? In these cases, we use a "-" in front of the number, but this data also needs to be stored. Generally, it can be assumed, that if there is a chance for the type to be negative (e.g. it is balanced) we must have a separate piece of information to signal whether it is positive or negative. This is generally true for all types, except for Balanced Ternary.

Let's go through a quick example for balanced ternary, let's start by writing the number 33 and splitting it off into it's separate parts:
```text
33 = P (1 * 3^3 = 27) P (1 * 3^2 = 9) N (-1 * 3^1 = -3) Z (0 * 3^0 = 0)
33 = 27 + 9 + (-3) + 0
```

Okay, that looks nice and all, but what about a negative number? For that we can use the number -10 and do the same evaluation:
```text
-10 = N (-1 * 3^2 = -9) Z (0 * 3^1 = 0) N (-1 * 3^0 = -1)
-10 = (-9) + 0 + (-1)
```

I won't dance around it too much, but balanced ternary is really pretty for the way signing is incorporated directly into the type. And it opens up for a lot of easy integrations and arithmetics. A good example of this is how to read sign, absolute value, negation and more.

## Sign
To check for a ternary words sign, it's trivially easy. You just check what the sign of the first non zero trit is. This logic works, as the first non zero trit will always be the most significant trit. You can have a number like NPPPPP (-122) and it will still be true, since the first N, represents -243, which dwarfs all the other Ps.

Here's the current code for how sign is extracted for words[^1]:
```rust,ignore
/// Returns the sign (whether it's negative, positive or zero) of the word
pub fn sign(&self) -> Trit {
    // For each t (trit) in own trit array (reversed to fit most to least significant logic)
    for t in self.0.iter().rev() {
        // If the checked trit isn't Z, then return and escape early
        if *t != Trit::Z {
            return *t;
        }
    }

    // If none are found, return Z
    Trit::Z
}
```

[^1]: [word.rs github source](https://github.com/silvijai/vildrose/blob/main/crates/vildrose-core/src/word.rs)

It also has the escape of returning zero, which another point of evidence for where trits, kleenes and ternary logic is so clean. Because if it had to return a boolean value, we'd be in trouble, then we'd have to decide whether 0 is positive or negative. Which isn't a true understanding. Likely you'd return it as a positive value, but truly, it does more so represent a non value.

## Negate
This is another situation where balanced ternary arithmetics are really clean. To negate a word in ternary, all you have to do is flip the sign of every trit in the word. This will leave you with the exact same number, just with the opposite sign. And for 0, nothing changes of course.

The code for this is also quite simple[^1]:
```rust,ignore
/// Returns the inverted (negated) form of the word
pub fn negate(&self) -> Self {
    // For each trit in word, map it's trit to the negated form of the trit
    Self(self.0.map(|t| t.negate()))
}
```

The .negate() function itself, is defined like this[^2]:
```rust,ignore
/// Return the opposite (negated) for a trit
pub const fn negate(self) -> Self {
    // Matches self (input) to find and return negated form
    match self {
        Self::N => Self::P,
        Self::Z => Self::Z,
        Self::P => Self::N,
    }
}
```

[^2]: [trit.rs github source](https://github.com/silvijai/vildrose/blob/main/crates/vildrose-core/src/trit.rs)

## Absolute
Getting the absolute form of a word is as simple as checking whether the sign is negative, and if it is, you negate the whole word. This can be done really simply in code with an if statement, and referencing back to the negate function[^1]:
```rust,ignore
/// Returns the absolute (no negatives) form of the word
pub fn abs(&self) -> Self {
    // If the words sign is N
    if self.sign() == Trit::N {
        // then negate
        self.negate()
    } else {
        // otherwise return itself
        *self
    }
}
```

## Addition


## Subtraction


## Multiplication


## Division
### Checked


### Unchecked
