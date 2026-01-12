
# Borrowing

Borrowing is a way of temporarily accessing data without taking ownership of it.
When borrowing, you're taking a reference (pointer) to the data, not the data itself.
The primary goal is to prevent dangling pointers and data races. 
Data can be borrowed immutably and mutably.
There are certain rules when borrowing which we have to comply with, 
otherwise the program won't compile.

## Rules of References
- At any given time, you can have either **one mutable reference** 
- or **any number of immutable references** but not both.
- References must **always be valid**.

