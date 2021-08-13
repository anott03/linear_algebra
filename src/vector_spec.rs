#[cfg(test)]
mod vector_spec {
    use crate::vector::vector;

    #[test]
    fn dot_prodcut_success() {
        let v: Vec<i32> = vec![3, 1, 2];
        let w: Vec<i32> = vec![1, 2, 3];
        let result = vector::dot_product(&v, &w).unwrap();

        assert_eq!(11, result);
    }

    #[test]
    fn dot_prodcut_failure() {
        let v: Vec<i32> = vec![3, 1, 2, 4];
        let w: Vec<i32> = vec![1, 2, 3];

        assert!(vector::dot_product(&v, &w).is_err());
    }

    #[test]
    fn projection_success() {
        let v: Vec<i32> = vec![1, -1, 1];
        let w: Vec<i32> = vec![1, 1, 1];

        let correct_projection: Vec<i32> = vec![2/3, 2/3, 2/3];
        let calculated_projection = vector::projection(&v, &w).unwrap();

        assert_eq!(correct_projection, calculated_projection);
    }

    #[test]
    fn projection_failure() {
        let v: Vec<i32> = vec![3, 1, 2, 4];
        let w: Vec<i32> = vec![1, 2, 3];

        assert!(vector::dot_product(&v, &w).is_err());
    }

    #[test]
    fn projection_basis_success() {
        let v = vec![3, 1, 2];
        let basis = vec![vec![1, 1, 1], vec![1, -1, 0]];

        let result = vector::projection_onto_basis(&v, &basis).unwrap();
        assert_eq!(result, v);
    }

    #[test]
    fn projection_basis_not_orthogonal() {
        let v = vec![3, 1, 2];
        let basis = vec![vec![1, 1, 1], vec![1, 1, 0]];
        assert!(vector::projection_onto_basis(&v, &basis).is_err());
    }
}
