# rejoin_slice

This crate provides functions to join two slices that are adjacent in memory.
It is useful for rejoining slices that are split from the same slice,
but need to be processed as a continous slice later:
```
let mut values: Vec<_> = util_lib::split_by_streak("aaaaaaabbbbbbbcccccccddddeeeeeeefffggggggggh");
let last_two = rejoin_str(&values[values.len()-2], &values[values.len()-1]);
assert_eq!("ggggggggh", last_two);
```
## Notes about safety
This crate internally uses `unsafe` to achieve its functionality.
However, it provides a safe interface.
It takes the following precautions for safety:
1. Pointer arithmetic is never explicitly performed. A pointer pointing to
the end of the first slice is calculated using safe API's.
2. Equality comparisons between pointers, although undefined behaviour in C in
cases where the pointers originate from different objects, can be considered
to be safe in Rust. This is ensured by the fact that the standard library
provides a safe function `std::ptr::eq` to compares pointers.
3. `unsafe` is only used to call `std::slice::from_raw_parts` to create a new
slice after the check that the input slices are adjacent in memory.
