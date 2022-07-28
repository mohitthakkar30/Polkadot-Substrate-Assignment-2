// 1.

/* Make it work, only using comments! */
fn main() {
    //todo!();
    //unimplemented!();

    assert_eq!(6, 5 + 3 + 2 + 1 )
}


// 2.

#![allow(unused)]
fn main() {
// in lib.rs

/// Add one to the given value and return the value
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
}


// 3.

#![allow(unused)]
fn main() {
/** Add two to the given value and return a new value

Examples

let arg = 5;
let answer = my_crate::add_two(arg);

assert_eq!(7, answer);

*/
pub fn add_two(x: i32) -> i32 {
    x + 2
}
}

// 4.

#![allow(unused)]
fn main() {
//! # Doc comments
//! 
//! A library for showing how to use doc comments

// in lib.rs
pub mod compute;
}
