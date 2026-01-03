
# Ownership

- Rust's ownership system is unique and sets it apart from other programming languages.
- Set of rules that govern memory management.
- Rules are enforced at compile time.
- If any of the rules are violated, the program won't compile.

## Three Rules of Ownership

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

**Owner**: The owner of a value is the variable or data structure that holds
it and is responsible for allocating and freeing the memory used to store that
data.

## Scope

Scope is the range within a program for which an item is valid

- **Global scope**: Accessible throughout the entire program.
- **Local scope**: Accessible only within particular function or block of code.

```rust

fn main() {

    {

        // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward
        
        // we can do stuff with s

    } // the scope is now over, and s is no longer valid
}

```

In the code snippet above:
- When `s` comes into scope, it is valid.
- It remains valid until it goes out of scope.
- **General rule:** Scope ends where block of code ends (curly brackets).


## Memory

To understand ownership in Rust, you must understand how data is stored in 
the computer memory.

- Component in a computer to store data and instructions for the processor to execute.
- Random Access Memory (RAM) is volatile, when power turned off all contents are lost.
- Two types of regions in RAM used by a program at runtime: *stack memory* and *heap memory*.

### Stack Memory

- Last in, first out.
- All data stored on the stack must have a known, fixed size (like integers, floats, char, bool etc.)
- Pushing to the stack is faster than allocating on the heap, because the location for new data is always at the top of the stack.
- Types of unknown size will get allocated to the heap and a pointer to the value is pushed to the stack, because a pointer is fixed size (usize)

### Heap Memory

- Data of no known, fixed size belongs on the heap.
- Allocating data on the heap will return a pointer (an address to location where data has been allocated)
- Allocating on the heap is slower than pushing to stack
- Accessing data on the heap is also slower, as it has to be accessed using a pointer which points to an address.

#### The String Type

All types covered so far were fixed size, and thus, are stored in the stack. 
The `String` type is used to store texts.

- `String` is mutable
- `String` size can change at runtime
- `String` stored on the stack with a pointer to the heap
- Value of `String` is stored on the heap

*NB*: A pointer in Rust takes up **24 bytes**, which is **8 bytes** (`usize`) each for the 
`ptr` for the memory address value, `len` for the length (of the string), and 
`capacity` for the capacity.

## Copy Vs. Move

- Scalar values with fixed sizes (all types we covered at the beginning) will automatically get copied in the stack, 
copying here is cheap.
- Dynamically sized data won't get copied, but moved, copying would be too expensive.

```rust

fn main() {
    
    // Here, the integer value of variable `x` will get copied to `y` and
    // both variables are usable, because i32 value has been copied
    // (this is because i32 is fixed)
    let x = 5;
    let y = x;
    
    // As `s1` is just a pointer to data on the heap, just the pointer will
    // get copied into `s2`, **NOT** the whole data on the heap!
    let s1 = String::from("hello");
    let s2 = s1;

```

In the code snippet above:
- y and x are stored in a different location, but the value is copied over to `y`
- here the pointer is copied, since it is of fixed size. This means that both `s1` and `s2`
will now point to the same memory location while the value (which is has varying size)
still retains it's location.

The implication is that this would **violate the second rule** which states that there can only
be **ONE owner** at a time. To prevent this, the first variable `s1` will be `dropped` and cannot
be used after assigning it to `s2`, to avoid dangling pointers. Thus, `s2` becomes the sole owner
of the memory address and `s1` becomes invalid from that point.

If we really want to copy the value from `s1` to `s2` so that both `s1` and `s2` point to different
memory address location.

```rust

fn main() {

    let s1 = String::from("hello!");
    let s2 = s1.clone();
    
    println!("s1 = {}, s2 = {}", s1, s2);
    
}

```

The following code snippet describes how ownership works in functions.

```rust

fn main() {
    let s = String::from("hello");      // s comes into scope

    takes_ownership(s);                 // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                          // x comes into scope

    makes_copy(x);                      // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward

}   // Hee, x goes out of scope, then s. But because s's value was moved, 
    // nothing special happens

fn takes_ownership(some_string: String) {   // some_string comes into scope
    println!("{}", some_string);
}   // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

fn makes_copy(some_integer: i32) {          // some_integer comes into scope
    println!("{}", some_integer);
}   // Here, some some_integer goes out of scope. Nothing special happens.


```

In the following snippets, we can see that we can return the value to the 
caller:

```rust

fn main() {
    
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    
    let s2 = String::from("hello");     // s2 comes into scope
    
    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
}   // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens, s1 goes out of scoped and is dropped.

fn gives_ownership() -> String {        // gives_ownership will move its
                                        // return value into the function
                                        // that calls it
    
    let some_string = String::from("yours");    // some_string comes into scope
    
    some_string;                                // some_String is returned and
                                                // moves out to the calling function
    
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into
                                                        // scope
    a_string // a_string is returned and moves out to the calling function
}

```


## Advantages of Ownership

The concept of ownership prevents memory safety issues:

- Dangling pointer
- Double-free: Trying to free memory that has already been freed.
- Memory leaks: Not freeing memory that should have been freed.

