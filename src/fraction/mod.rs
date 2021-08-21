mod spec;

#[allow(dead_code)]
pub mod fraction {
    use std::ops;

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

    impl ops::Add for Fraction {
        type Output = Fraction;
        fn add(self, other: Self) -> Self::Output {
            if self.denominator != other.denominator {
                return simplify(&Fraction {
                    numerator: self.numerator * other.denominator + other.numerator * self.denominator,
                    denominator: self.denominator * other.denominator
                });
            }

            return simplify(&Fraction {
                numerator: self.numerator + other.numerator,
                denominator: self.denominator,
            });
        }
    }

    impl ops::Add<i32> for Fraction {
        type Output = Fraction;
        
        fn add(self, rhs: i32) -> Fraction {
            return Fraction {
                numerator: self.numerator + rhs * self.denominator,
                denominator: self.denominator
            }
        }
    }

    impl ops::Sub for Fraction {
        type Output = Fraction;

        fn sub(self, rhs: Self) -> Self::Output {
            return self + (rhs * -1);
        }
    }

    impl ops::Sub<i32> for Fraction {
        type Output = Fraction;

        fn sub(self, rhs: i32) -> Self::Output {
            return self + (rhs * -1);
        }
    }

    impl ops::Mul for Fraction {
        type Output = Fraction;

        fn mul(self, rhs: Self) -> Self::Output {
            return simplify(&Fraction {
                numerator: self.numerator * rhs.numerator,
                denominator: self.denominator * rhs.denominator,
            })
        }
    }

    impl ops::Mul<i32> for Fraction {
        type Output = Fraction;

        fn mul(self, rhs: i32) -> Self::Output {
            return simplify(&Fraction {
                numerator: self.numerator * rhs,
                denominator: self.denominator,
            })
        }
    }

    impl ops::Div for Fraction {
        type Output = Fraction;

        fn div(self, rhs: Self) -> Self::Output {
            return simplify(&Fraction::new(
                self.numerator * rhs.denominator,
                self.denominator * rhs.numerator
            ));
        }
    }

    impl ops::Div<i32> for Fraction {
        type Output = Fraction;
        fn div(self, rhs: i32) -> Self::Output {
            return self * Fraction::new(1, rhs);
        }
    }

    pub fn simplify(frac: &Fraction) -> Fraction {
        let gcd = greatest_common_factor(frac.numerator, frac.denominator);

        return Fraction {
            numerator: frac.numerator / gcd,
            denominator: frac.denominator / gcd,
        }
    }

    // should this be public...?
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
}
