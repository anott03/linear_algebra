#[cfg(test)]
mod tests {
    use crate::fraction::fraction;

    #[test]
    fn test() {
        let f = fraction::Fraction::new(1, 3);
        assert!(1 == f.numerator && 3 == f.denominator);
    }
}
