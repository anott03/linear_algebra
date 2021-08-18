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

    pub fn greatest_common_factor(a: i32, b: i32) -> i32 {
        // variable names based off Euclidean divison equation: a = b · q + r
        let (mut a, mut b) = if a > b {
            (a, b)
        } else {
            (b, a)
        };

        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }

        return a
    }

    pub fn simplify(frac: &Fraction) -> Fraction {
        let gcd = greatest_common_factor(frac.numerator, frac.denominator);

        return Fraction {
            numerator: frac.numerator / gcd,
            denominator: frac.denominator / gcd,
        }
    }
}
