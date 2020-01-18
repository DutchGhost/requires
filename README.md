# Requires
Procedural macros for requiring const generics to meet some requirement.
The requirement can be any arbitrary code, but must result in a boolean expression.
This expression can be as complex as possible, it only has to be const evaluatable.

There's two macro's that make this happen:
 - `#[requires]`
 - `#[validate]`

`#[requires]` sets op the requirement for the const generics.
In order to validate whether the const generics indeed meet the requirement, use `#[validate]` on the constructor for the type, e.g:
```Rust
#![feature(const_generics)]

use requires::*;

// Setup the requirement for P and Q.
#[requires(P > 10 && Q < 10)]
struct Example<const P :usize, const Q: usize> {
    n: usize
}

impl <const P: usize, const Q: usize> Example<{P}, {Q}> {
    // Validate P and Q in the constructor.
    #[validate]
    pub fn new<T: Into<usize>>(n: T) -> Self {
        Self::validate();

        Self { n: n.into() }
    }
}

fn main() {
    Example::<{11}, {9}>::new(0usize);

    // DOESNT COMPILE:
    // Example::<{9}, {9}>::new(1usize);
}
```

or another more complex example:
```Rust
#![feature(const_generics)]

use requires::*;

#[requires(!S.is_empty() && S.as_bytes()[0] == b'H')]
struct Example<const S: &'static str> {
    n: usize,
}
```