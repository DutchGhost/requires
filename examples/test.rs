#![feature(const_generics)]

use requires::*;

#[requires(P > 10 && Q < 10)]
struct Validate<const P :usize, const Q: usize> {
    n: usize
}

impl <const P: usize, const Q: usize> Validate<{P}, {Q}> {
    pub fn new(n: usize) -> Self {
        Self::validate();

        Self { n }
    }
}

fn main() {
    Validate::<{11}, {9}>::new(0);
}