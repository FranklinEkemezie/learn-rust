# Numbers
## Integer Types

- **Signed integer**: Can represent both **positive** and **negative** integers
- **Unsigned integer**: Always **positive** integers

| Length  | Signed | Unsigned |
|---------|:------:|:--------:|
| 8-bit   |   i8   |    u8    |
| 16-bit  |  i16   |   u16    |
| 32-bit  |  i32   |   u32    |
| 64-bit  |  i64   |   u64    |
| 128-bit |  i128  |   u128   |
| arch    | isize  |  usize   |

## Default Types
When no type is specified, the following default types are used:
- Integers: `i32`
- Floats: `f64`

## Range of Integers
### 8-bit integers
- **Unsigned**: `0` - `255`
- **Signed**: `-128` - `127`
### 16-bit integers
- **Unsigned**: `0` - `65,535`
- **Signed**: `-32,768` - `32,767`

### `usize` & `isize` integer types
- Architecture dependent
- On 32-bit architecture: **32-bit**
- On 64-bit architecture: **64-bit**
- Also called pointer sized integer type, matches size of a **word** in given platform.

## What is a word?
- Processor does NOT read 1 byte at a time from memory, reads 1 word at a time.
- In a **32-bit** processor it can access 4 bytes (32 bits) at a time.
- In a **64-bit** processor it can access 8 bytes (64 bits) at a time.
- Since `usize` is always equal to the **word size** for any architecture, it is
guaranteed to be big enough to hold any **pointer** or any offset in a data structure, hence
the name **pointer-sized integer** because it can hold exactly the amount of data the processor
will read at any time.

## Floating point?
- `f32` - size of 32 bits.
- `f64` - size of 64 bits.
- Representation according to IEEE-754 specification.

## Boolean Logic
- Boolean logic deals with values that are either "true" or "false" (or 1 and 0).
- Three basic operations: AND, OR, NOT.

### Truth Table AND
| A     |   B   | Result |
|-------|:-----:|:------:|
| false | false | false  |
| false | true  | false  |
| true  | false | false  |
| true  | true  |  true  |

### Truth Table OR
| A     |   B   | Result |
|-------|:-----:|:------:|
| false | false | false  |
| false | true  |  true  |
| true  | false |  true  |
| true  | true  |  true  |

### Truth Table NOT
| A       | Result |
|---------|:------:|
| ! false |  true  |
| ! true  | false  |

## Bitwise Operations
- Operations that manipulate individual bits that make up a binary number.
- Treating each bit of a binary number as a separate unit and perform logical operations on them.
- Bitwise operations include: AND, OR, XOR, bitwise shifting.

# AND (&)
**AND** returns 1 only when **both** of its input are 1.

| A | B | Q |
|---|:-:|:-:|
| 0 | 0 | 0 |
| 0 | 1 | 0 |
| 1 | 0 | 0 |
| 1 | 1 | 0 |

# OR (|)
**OR** returns 1 if **at least** one of its inputs is 1. 
If both inputs are 0, the output will also be 0.

| A | B | Q |
|---|:-:|:-:|
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 1 |

# XOR (^)
**XOR** or (exclusive OR) returns 1 if the inputs are **different** 
and 0 if the inputs are the **same**.

| A | B | Q |
|---|:-:|:-:|
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

# Bitwise Left Shift

Ex: To perform `1 << 5`

*Solution*

1 `->` 0000_0001

Now, shift all the bits to the left 5 times, we get:
`=>` `0010_0000`
which is 32.

# Bitwise Right Shift

Ex: To perform `0x80 >> 2`

*Solution*

Here, `0x80` is the same as `128` in decimal.

128 `->` 1000_0000

Now, shift all the bits to the right 5 times, we get:
`=>` `0010_0000`
which is 32 (or `0x20` in octal).


