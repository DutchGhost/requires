use requires::*;

#[requires(P > 10 && Q == P)]
#[requires(even = P & 1 == 0)]
#[requires(uneven = P & 1 == 1)]
pub struct Validate<const P: usize, const Q: usize> {
    n: usize,
}

#[validate]
impl<const P: usize, const Q: usize> Validate<{ P }, { Q }> {
    pub fn new<T: Into<usize>>(n: T) -> Self {
        Self { n: n.into() }
    }

    #[validate(even)]
    pub fn blah() -> usize {
        1
    }

    #[validate(uneven)]
    pub fn blah2() -> usize {
        1
    }
}

fn main() {
    //Validate::<{ 11 }, { 11 }>::new(0usize);

    dbg!("DONE");
    Validate::<{ 12 }, { 12 }>::blah();
    Validate::<{ 11 }, { 11 }>::blah2();
}
