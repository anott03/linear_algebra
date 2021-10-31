#[cfg(test)]
mod spec {
    use crate::transformation::transformation;

    #[test]
    fn test() {
        let x: f64 = std::f64::consts::FRAC_PI_4;
        assert_eq!(x.tan(), x.sin() / x.cos());
    }

    #[test]
    fn rotation1() {
        let v = vec![1, 0];
        assert_eq!(transformation::rotate(&v, std::f64::consts::FRAC_PI_2), vec![0, 1])
    }
}
