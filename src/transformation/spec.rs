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
        let v: Vec<f64> = vec![1.0, 0.0];
        assert_eq!(transformation::rotate(&v, std::f64::consts::FRAC_PI_2).unwrap(), vec![0.0, 1.0])
    }
}
