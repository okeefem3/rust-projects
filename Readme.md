# Rust learning code for Michael


Rust is statically typed - Must know types of vars at compile time either through annotation or inference

Scalar types (single value types) -
integers, floating point, booleans, characters

## Integers
either signed or unsigned
i8 - i128 or u8 - u128
also arch - (isize, usize) which uses the default architecture size

Two's complement is used for storage:

Each signed variant can store numbers from -(2^(n - 1)) to 2^(n - 1) - 1 inclusive, where n is the number of bits that variant uses

Unsigned variants can store numbers from 0 to 2^(n - 1)

The primary situation in which you’d use isize or usize is when indexing some sort of collection.

Otherwise using i32 is generally the fastest, and is the default
