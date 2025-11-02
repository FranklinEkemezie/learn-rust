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


