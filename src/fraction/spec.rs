#[cfg(test)]
mod tests {
    use crate::fraction::fraction;

    #[test]
    fn gcd() {
        assert_eq!(fraction::greatest_common_factor(4, 16), 4);
    }

    #[test]
    fn simplify() {
        let f = fraction::Fraction::new(4, 16);
        let simplified_f = fraction::simplify(&f);
        assert!(simplified_f.numerator == 1 && simplified_f.denominator == 4);
    }

    #[test]
    fn multiply() {
        let a = fraction::Fraction::new(1, 3);
        let b = fraction::Fraction::new(1, 4);
        let expected = fraction::Fraction::new(1, 12);
        let calculated_answer = a * b;

        assert_eq!(expected, calculated_answer);
    }

    #[test]
    fn add1() {
        let a = fraction::Fraction::new(1, 2);
        let b = fraction::Fraction::new(2, 2);
        let expected = fraction::Fraction::new(3, 2);
        let actual = a + b;
        assert_eq!(expected, actual);
    }

    #[test]
    fn add2() {
        let a = fraction::Fraction::new(1, 2);
        let b = fraction::Fraction::new(1, 4);
        let expected = fraction::Fraction::new(3, 4);
        let actual = a + b;
        assert_eq!(expected, actual);
    }

    #[test]
    fn subtract1() {
        let a = fraction::Fraction::new(3, 4);
        let b = fraction::Fraction::new(1, 4);
        let expected = fraction::Fraction::new(1, 2);
        let actual = a - b;
        assert_eq!(expected, actual);
    }

    #[test]
    fn division1() {
        let a = fraction::Fraction::new(1, 2);
        let b = fraction::Fraction::new(1, 4);
        let expected = fraction::Fraction::new(2, 1);
        let actual = a / b;
        assert_eq!(expected, actual);
    }

    #[test]
    fn division2() {
        let a = fraction::Fraction::new(1, 2);
        let expected = fraction::Fraction::new(1, 4);
        let actual = a / 2;
        assert_eq!(expected, actual);
    }
}
