#[cfg(test)]
mod tests {
    use crate::vector::vector;

    #[test]
    fn dot_prodcut_success() {
        let v: Vec<f64> = vec![3.0, 1.0, 2.0];
        let w: Vec<f64> = vec![1.0, 2.0, 3.0];
        let result = vector::dot_product(&v, &w).unwrap();

        assert_eq!(11.0, result);
    }

    #[test]
    fn dot_prodcut_failure() {
        let v: Vec<f64> = vec![3.0, 1.0, 2.0, 4.0];
        let w: Vec<f64> = vec![1.0, 2.0, 3.0];

        assert!(vector::dot_product(&v, &w).is_err());
    }

    #[test]
    fn projection_success() {
        let v: Vec<f64> = vec![1.0, -1.0, 1.0];
        let w: Vec<f64> = vec![1.0, 1.0, 1.0];

        let correct_projection: Vec<f64> = vec![1.0/3.0, 1.0/3.0, 1.0/3.0];
        let calculated_projection = vector::projection(&v, &w).unwrap();

        assert_eq!(correct_projection, calculated_projection);
    }

    #[test]
    fn projection_failure() {
        let v: Vec<f64> = vec![3.0, 1.0, 2.0, 4.0];
        let w: Vec<f64> = vec![1.0, 2.0, 3.0];

        assert!(vector::dot_product(&v, &w).is_err());
    }

    #[test]
    fn projection_basis_success() {
        let v = vec![3.0, 1.0, 2.0];
        let basis = vec![vec![1.0, 1.0, 1.0], vec![1.0, -1.0, 0.0]];

        let result = vector::projection_onto_basis(&v, &basis).unwrap();
        assert_eq!(result, v);
    }

    #[test]
    fn projection_basis_success2() {
        let v = vec![4.0, 7.0, 9.0];
        let basis = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]];
        let result = vector::projection_onto_basis(&v, &basis).unwrap();
        assert_eq!(result, v);
    }

    #[test]
    fn projection_basis_not_orthogonal() {
        let v = vec![3.0, 1.0, 2.0];
        let basis = vec![vec![1.0, 1.0, 1.0], vec![1.0, 1.0, 0.0]];
        assert!(vector::projection_onto_basis(&v, &basis).is_err());
    }

    #[test]
    fn check_orthogonal_basis_true() {
        let basis = vec![vec![1.0, 1.0, 1.0], vec![1.0, -1.0, 0.0]];
        assert_eq!(vector::check_orthogonal_basis(&basis), true);
    }

    #[test]
    fn check_orthogonal_basis_false() {
        let basis = vec![vec![1.0, 1.0, 1.0], vec![1.0, 1.0, 0.0]];
        assert_eq!(vector::check_orthogonal_basis(&basis), false);
    }

    #[test]
    fn cross_product_failure() {
        let v = vec![1.0, 2.0];
        let w = vec![2.0, 3.0, 4.0];
        assert!(vector::cross_product(&v, &w).is_err());
    }

    #[test]
    fn cross_product_success() {
        let v = vec![1.0, 0.0, 0.0];
        let w = vec![0.0, 1.0, 0.0];
        assert_eq!(vector::cross_product(&v, &w).unwrap(), vec![0.0, 0.0, 1.0]);
    }
}
