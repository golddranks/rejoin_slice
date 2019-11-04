//! This crate provides methods for joining two slices that are adjacent in memory.
//! It is useful for rejoining slices that are split from the same slice,
//! but need to be processed as a continous slice later:
//! ```
//! # use rejoin_slice::StrExt;
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
//! let last_two = &values[values.len()-2].rejoin(&values[values.len()-1]);
//! assert_eq!(&"ggggggggh", last_two);
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

pub trait SliceExt {
    /// Joins two slices that are adjacent in memory into one slice.
    /// # Panics
    /// Panics in the case the slices aren't adjacent.
    fn rejoin<'r>(&'r self, other: &'r Self) -> &'r Self;

    /// Joins two mutable slices that are adjacent in memory into one slice.
    /// # Panics
    /// Panics in the case the slices aren't adjacent.
    fn rejoin_mut<'r>(&'r mut self, other: &'r mut Self) -> &'r mut Self;

    /// Joins two slices that are adjacent in memory into one slice.
    /// Returns None in the case the slices aren't adjacent.
    fn try_rejoin<'r>(&'r self, other: &'r Self) -> Option<&'r Self>;

    /// Joins two mutable slices that are adjacent in memory into one slice.
    /// Returns None in the case the slices aren't adjacent.
    fn try_rejoin_mut<'r>(&'r mut self, other: &'r mut Self) -> Option<&'r mut Self>;
}

impl<T> SliceExt for [T] {
    fn rejoin<'r>(&'r self, other: &'r [T]) -> &'r [T] {
        self.try_rejoin(other).expect("the input slices must be adjacent in memory")
    }

    fn rejoin_mut<'r>(&'r mut self, other: &'r mut [T]) -> &'r mut [T] {
        self.try_rejoin_mut(other).expect("the input slices must be adjacent in memory")
    }

    fn try_rejoin<'r>(&'r self, other: &'r [T]) -> Option<&'r [T]> {
        let self_len = self.len();
        let self_end = self[self_len..].as_ptr();
        if core::ptr::eq(self_end, other.as_ptr()) {
            Some(unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len() + other.len()) })
        } else {
            None
        }
    }

    fn try_rejoin_mut<'r>(&'r mut self, other: &'r mut [T]) -> Option<&'r mut [T]> {
        let self_len = self.len();
        let self_end = self[self_len..].as_mut_ptr();
        if core::ptr::eq(self_end, other.as_mut_ptr()) {
            Some(unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len() + other.len()) })
        } else {
            None
        }
    }
}

pub trait StrExt {
    /// Joins two string slices that are adjacent in memory into one string slice.
    /// # Panics
    /// Panics in the case the slices aren't adjacent.
    fn rejoin<'r>(&'r self, other: &'r str) -> &'r str;

    /// Joins two string slices that are adjacent in memory into one string slice.
    /// Returns None in the case the slices aren't adjacent.
    fn try_rejoin<'r>(&'r self, other: &'r str) -> Option<&'r str>;
}

impl StrExt for str {
    fn rejoin<'r>(&'r self, other: &'r str) -> &'r str {
        self.try_rejoin(other).expect("the input string slices must be adjacent in memory")
    }

    fn try_rejoin<'r>(&'r self, other: &'r str) -> Option<&'r str> {
        self.as_bytes().try_rejoin(other.as_bytes()).map(|s| unsafe { core::str::from_utf8_unchecked(s) })
    }
}


#[test]
fn test_rejoin() {
    let slice = &[0, 1, 2, 3, 4, 5, 6][..];

    assert_eq!(slice[..3].rejoin(&slice[3..]), slice);
    assert_eq!(slice[..4].rejoin(&slice[4..]), slice);
    assert_eq!(slice[..0].rejoin(&slice[0..]), slice);
    assert_eq!(slice[..1].rejoin(&slice[1..]), slice);
    assert_eq!(slice[..6].rejoin(&slice[6..]), slice);
    assert_eq!(slice[..7].rejoin(&slice[7..]), slice);
}

#[test]
#[should_panic]
fn test_rejoin_nogaps() {
    let slice = &[0, 1, 2, 3, 4, 5, 6][..];

    // Don't allow gaps between slices
    slice[..3].rejoin(&slice[4..]);
}
#[test]
#[should_panic]
fn test_rejoin_leftright() {
    let slice = &[0, 1, 2, 3, 4, 5, 6][..];

    // Don't allow joining in wrong order
    slice[3..].rejoin(&slice[..3]);
}

#[test]
fn test_rejoin_mut() {
    let slice = &mut [0, 1, 2, 3, 4, 5, 6][..];

    let (a, b) = slice.split_at_mut(3);
    a.rejoin_mut(b).copy_from_slice(&[14, 15, 16, 17, 18, 19, 20][..]);
    assert_eq!(slice, &[14, 15, 16, 17, 18, 19, 20][..]);

    let (a, b) = slice.split_at_mut(4);
    a.rejoin_mut(b).copy_from_slice(&[21, 22, 23, 24, 25, 26, 27][..]);
    assert_eq!(slice, &[21, 22, 23, 24, 25, 26, 27][..]);

    let (a, b) = slice.split_at_mut(0);
    a.rejoin_mut(b).copy_from_slice(&[28, 29, 30, 31, 32, 33, 34][..]);
    assert_eq!(slice, &[28, 29, 30, 31, 32, 33, 34][..]);

    let (a, b) = slice.split_at_mut(1);
    a.rejoin_mut(b).copy_from_slice(&[35, 36, 37, 38, 39, 40, 41][..]);
    assert_eq!(slice, &[35, 36, 37, 38, 39, 40, 41][..]);

    let (a, b) = slice.split_at_mut(6);
    a.rejoin_mut(b).copy_from_slice(&[42, 43, 44, 45, 46, 47, 48][..]);
    assert_eq!(slice, &[42, 43, 44, 45, 46, 47, 48][..]);

    let (a, b) = slice.split_at_mut(7);
    a.rejoin_mut(b).copy_from_slice(&[49, 50, 51, 52, 53, 54, 55][..]);
    assert_eq!(slice, &[49, 50, 51, 52, 53, 54, 55][..]);
}

#[test]
#[should_panic]
fn test_rejoin_mut_nogaps() {
    let slice = &mut [0, 1, 2, 3, 4, 5, 6][..];

    // Don't allow gaps between slices
    let (a, b) = slice.split_at_mut(3);
    a.rejoin_mut(&mut b[1..]);
}
#[test]
#[should_panic]
fn test_rejoin_mut_leftright() {
    let slice = &mut [0, 1, 2, 3, 4, 5, 6][..];

    // Don't allow joining in wrong order
    let (a, b) = slice.split_at_mut(3);
    b.rejoin_mut(a);
}

#[test]
fn test_try_rejoin() {
    let slice = &[0, 1, 2, 3, 4, 5, 6][..];

    assert_eq!(slice[..3].try_rejoin(&slice[3..]), Some(slice));
    assert_eq!(slice[..0].try_rejoin(&slice[0..]), Some(slice));
    assert_eq!(slice[..1].try_rejoin(&slice[1..]), Some(slice));
    assert_eq!(slice[..6].try_rejoin(&slice[6..]), Some(slice));
    assert_eq!(slice[..7].try_rejoin(&slice[7..]), Some(slice));

    assert_eq!(slice[..3].try_rejoin(&slice[4..]), None);
    assert_eq!(slice[3..].try_rejoin(&slice[3..]), None);
}

#[test]
fn test_try_rejoin_mut() {
    let slice = &mut [0, 1, 2, 3, 4, 5, 6][..];

    let (a, b) = slice.split_at_mut(3);
    a.try_rejoin_mut(b).as_mut().map(|s| s.copy_from_slice(&[14, 15, 16, 17, 18, 19, 20][..]));
    assert_eq!(slice, &[14, 15, 16, 17, 18, 19, 20][..]);

    let (a, b) = slice.split_at_mut(4);
    a.try_rejoin_mut(b).as_mut().map(|s| s.copy_from_slice(&[21, 22, 23, 24, 25, 26, 27][..]));
    assert_eq!(slice, &[21, 22, 23, 24, 25, 26, 27][..]);

    let (a, b) = slice.split_at_mut(0);
    a.try_rejoin_mut(b).as_mut().map(|s| s.copy_from_slice(&[28, 29, 30, 31, 32, 33, 34][..]));
    assert_eq!(slice, &[28, 29, 30, 31, 32, 33, 34][..]);

    let (a, b) = slice.split_at_mut(1);
    a.try_rejoin_mut(b).as_mut().map(|s| s.copy_from_slice(&[35, 36, 37, 38, 39, 40, 41][..]));
    assert_eq!(slice, &[35, 36, 37, 38, 39, 40, 41][..]);

    let (a, b) = slice.split_at_mut(6);
    a.try_rejoin_mut(b).as_mut().map(|s| s.copy_from_slice(&[42, 43, 44, 45, 46, 47, 48][..]));
    assert_eq!(slice, &[42, 43, 44, 45, 46, 47, 48][..]);

    let (a, b) = slice.split_at_mut(7);
    a.try_rejoin_mut(b).as_mut().map(|s| s.copy_from_slice(&[49, 50, 51, 52, 53, 54, 55][..]));
    assert_eq!(slice, &[49, 50, 51, 52, 53, 54, 55][..]);

    let (a, b) = slice.split_at_mut(3);
    assert_eq!(a.try_rejoin_mut(&mut b[1..]), None);

    let (a, b) = slice.split_at_mut(4);
    assert_eq!(b.try_rejoin_mut(a), None);
}

#[test]
fn test_str_rejoin() {
    let slice = &"abcdefg"[..];

    assert_eq!(slice[..3].rejoin(&slice[3..]), slice);
    assert_eq!(slice[..4].rejoin(&slice[4..]), slice);
    assert_eq!(slice[..0].rejoin(&slice[0..]), slice);
    assert_eq!(slice[..1].rejoin(&slice[1..]), slice);
    assert_eq!(slice[..6].rejoin(&slice[6..]), slice);
    assert_eq!(slice[..7].rejoin(&slice[7..]), slice);
}

#[test]
#[should_panic]
fn test_str_rejoin_nogaps() {
    let slice = &"abcdefg"[..];

    // Don't allow gaps between slices
    slice[..3].rejoin(&slice[4..]);
}
#[test]
#[should_panic]
fn test_str_rejoin_leftright() {
    let slice = &"abcdefg"[..];

    // Don't allow joining in wrong order
    slice[3..].rejoin(&slice[..3]);
}

#[test]
fn test_str_try_rejoin() {
    let slice = &"abcdefg"[..];

    assert_eq!(slice[..3].try_rejoin(&slice[3..]), Some(slice));
    assert_eq!(slice[..0].try_rejoin(&slice[0..]), Some(slice));
    assert_eq!(slice[..1].try_rejoin(&slice[1..]), Some(slice));
    assert_eq!(slice[..6].try_rejoin(&slice[6..]), Some(slice));
    assert_eq!(slice[..7].try_rejoin(&slice[7..]), Some(slice));

    assert_eq!(slice[..3].try_rejoin(&slice[4..]), None);
    assert_eq!(slice[3..].try_rejoin(&slice[3..]), None);
}
