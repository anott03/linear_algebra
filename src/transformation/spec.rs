#[cfg(test)]
mod spec {
    #[test]
    fn test() {
        let x: f64 = std::f64::consts::FRAC_PI_4;
        assert_eq!(x.tan(), x.sin() / x.cos());
    }
}
