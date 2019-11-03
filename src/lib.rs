//! This crate provides functions to join two slices that are adjacent in memory.
//! It is useful for rejoining slices that are split from the same slice,
//! but need to be processed as a continous slice later:
//! ```
//! # use rejoin_slice::rejoin_str;
//! # mod util_lib {
//! #    pub fn split_by_streak(input: &str) -> Vec<&str> {
//! #        if input.is_empty() { return Vec::new() };
//! #        let mut last_char = input.chars().next().unwrap();
//! #        let mut last_idx = 0;
//! #        let mut output = Vec::new();
//! #        for (i, c) in input.char_indices() {
//! #            if last_char != c {
//! #                output.push(&input[last_idx..i]);
//! #                last_char = c;
//! #                last_idx = i;
//! #            }
//! #        }
//! #        output.push(&input[last_idx..]);
//! #        output
//! #    }
//! # }
//! let mut values: Vec<_> = util_lib::split_by_streak("aaaaaaabbbbbbbcccccccddddeeeeeeefffggggggggh");
//! let last_two = rejoin_str(&values[values.len()-2], &values[values.len()-1]);
//! assert_eq!("ggggggggh", last_two);
//! ```
//! # Notes about safety
//! This crate internally uses `unsafe` to achieve its functionality.
//! However, it provides a safe interface.
//! It takes the following precautions for safety:
//! 1. Pointer arithmetic is never explicitly performed. A pointer pointing to
//! the end of the first slice is calculated using safe API's.
//! 2. Equality comparisons between pointers, although undefined behaviour in C in
//! cases where the pointers originate from different objects, can be considered
//! to be safe in Rust. This is ensured by the fact that the standard library
//! provides a safe function `core::ptr::eq` to compares pointers.
//! 3. `unsafe` is only used to call `core::slice::from_raw_parts` to create a new
//! slice after the check that the input slices are adjacent in memory.

#![no_std]

/// Joins two slices that are adjacent in memory into one slice.
/// The two input slices always outlive the returned slice.
/// # Panics
/// Panics in the case the slices aren't adjacent.
pub fn rejoin<'r, 'a: 'r, 'b: 'r, T>(a: &'a [T], b: &'b [T]) -> &'r [T] {
    try_rejoin(a, b).expect("the input slices must be adjacent in memory")
}

/// Joins two mutable slices that are adjacent in memory into one slice.
/// The two input slices always outlive the returned slice.
/// # Panics
/// Panics in the case the slices aren't adjacent.
pub fn rejoin_mut<'r, 'a: 'r, 'b: 'r, T>(a: &'a mut [T], b: &'b mut [T]) -> &'r mut [T] {
    try_rejoin_mut(a, b).expect("the input slices must be adjacent in memory")
}

/// Joins two string slices that are adjacent in memory int one string slice.
/// The two input slices always outlive the returned slice.
/// # Panics
/// Panics in the case the slices aren't adjacent.
pub fn rejoin_str<'r, 'a: 'r, 'b: 'r>(a: &'a str, b: &'b str) -> &'r str {
    try_rejoin_str(a, b).expect("the input string slices must be adjacent in memory")
}

/// Joins two slices that are adjacent in memory into one slice.
/// The two input slices always outlive the returned slice.
/// Returns None in the case the slices aren't adjacent.
pub fn try_rejoin<'r, 'a: 'r, 'b: 'r, T>(a: &'a [T], b: &'b [T]) -> Option<&'r [T]> {
    let a_len = a.len();
    let a_tail = a[a_len..].as_ptr();
    if core::ptr::eq(a_tail, b.as_ptr()) {
        Some(unsafe { core::slice::from_raw_parts(a.as_ptr(), a.len() + b.len()) })
    } else {
        None
    }
}

/// Joins two mutable slices that are adjacent in memory into one slice.
/// The two input slices always outlive the returned slice.
/// Returns None in the case the slices aren't adjacent.
pub fn try_rejoin_mut<'r, 'a: 'r, 'b: 'r, T>(a: &'a mut [T], b: &'b mut [T]) -> Option<&'r mut [T]> {
    let a_len = a.len();
    let a_tail = a[a_len..].as_mut_ptr();
    if core::ptr::eq(a_tail, b.as_mut_ptr()) {
        Some(unsafe { core::slice::from_raw_parts_mut(a.as_mut_ptr(), a.len() + b.len()) })
    } else {
        None
    }
}
/// Joins two string slices that are adjacent in memory int one string slice.
/// The two input slices always outlive the returned slice.
/// Returns None in the case the slices aren't adjacent.
pub fn try_rejoin_str<'r, 'a: 'r, 'b: 'r>(a: &'a str, b: &'b str) -> Option<&'r str> {
    try_rejoin(a.as_bytes(), b.as_bytes()).map(|s| unsafe { core::str::from_utf8_unchecked(s) })
}
