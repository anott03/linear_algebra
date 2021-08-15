mod spec;

pub mod fraction {
    pub struct Fraction {
        pub numerator: i32,
        pub denominator: i32
    }

    impl Fraction {
        pub fn new(n: i32, d: i32) -> Self {
            return Fraction {
                numerator: n,
                denominator: d
            }
        }
    }
}
