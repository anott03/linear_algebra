#[cfg(test)]
mod tests {
    use crate::fraction::fraction;

    #[test]
    fn test() {
        let f = fraction::Fraction::new(1, 3);
        assert!(1 == f.numerator && 3 == f.denominator);
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
}
