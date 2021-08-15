mod spec;

pub mod fraction {
    #[derive(Debug)]
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

    impl PartialEq for Fraction {
        fn eq(&self, other: &Self) -> bool {
            return self.numerator == other.numerator &&
                self.denominator == other.denominator;
        }
    }

    pub fn multiply(a: &Fraction, b: &Fraction) -> Fraction {
        return Fraction {
            numerator: a.numerator * b.numerator,
            denominator: a.denominator * b.denominator,
        }
    }

    pub fn simplify(frac: &Fraction) -> Fraction {
        return Fraction { // placeholder so rust analyzer doesn't get mad
            numerator: 1,
            denominator: 1,
        }
    }
}