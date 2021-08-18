#[cfg(test)]
mod tests {
    use crate::fraction::fraction;

    #[test]
    fn gcd() {
        assert_eq!(fraction::greatest_common_factor(4, 16), 4);
    }

    #[test]
    fn multiply() {
        let a = fraction::Fraction::new(1, 3);
        let b = fraction::Fraction::new(1, 4);
        let correct_answer = fraction::Fraction {
            numerator: 1,
            denominator: 12
        };
        let calculated_answer = fraction::multiply(&a, &b);

        assert_eq!(correct_answer, calculated_answer);
    }

    #[test]
    fn simplify() {
        let f = fraction::Fraction::new(4, 16);
        let simplified_f = fraction::simplify(&f);
        assert!(simplified_f.numerator == 1 && simplified_f.denominator == 4);
    }
}
