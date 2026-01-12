
# String vs. &str

- A string is a compound type. A compound type is a type that is a collection of one type.
- A string is made up of characters, thus it is a compound type.

There are two string types in Rust: 
- `String` (commonly called the string type) and 
- `&str` (commonly referred to as string slice).

## String vs. &str
- A `String` is a **heap-allocated** string type that **owns** its contents and is **mutable**.
- A `&str` is an **immutable** sequence of UTF-8 bytes in memory, it does *not own* the underlying data and is **immutable**.
- Think of `&str` as a **view** on a sequence of characters (Stored as UTF-8 bytes) in memory.
- Use `&str` if you just want a view of a string.
- `&str` is more lightweight and efficient than `String`
- Use `String` if you need to own the data and be able to mutate it.

## String Literal
- A string literal is a sequence of characters enclosed in double quotes ("")
- It is a fixed size, compile-time known sequence of UTF-8 bytes.
- The type is `&'static str`, which indicates the data is stored in **static storage**, meaning it is **valid** throughout the **entire lifetime** of the program.
- The data is **hardcoded** into the executable and stored in **read-only memory**, meaning they are **immutable**.
