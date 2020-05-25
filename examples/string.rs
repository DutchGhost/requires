#![feature(const_generics)]

use requires::*;

#[requires(!S.is_empty() && S.as_bytes()[0] == b'H')]
struct Example<const S: &'static str> {
    n: usize,
}

#[validate]
impl<const S: &'static str> Example<{ S }> {
    pub fn new(n: usize) -> Self {
        Self { n }
    }

    pub fn test() {}
}

fn main() {
    Example::<{ "Hello world!" }>::new(10);
}
