//! # rustz
//!
//! Rustz is a collection of useful Rust utilities for functional programming.
mod combine;

use combine::*;

struct Type1;
struct Type2;
struct Type3;

fn main() {
    let result1: Result<Type1, &str> = Ok(Type1);
    let result2: Result<Type2, &str> = Ok(Type2);
    let result3: Result<Type3, &str> = Ok(Type3);
    let merged1 = hoge().into_merge().merge(result2).merge(result3);
}

fn hoge() -> Result<String, &'static str> {
    Ok("hoge".to_string())
}
