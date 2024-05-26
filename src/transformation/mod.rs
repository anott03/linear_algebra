mod spec;

#[allow(dead_code)]
pub mod transformation {
    // use std::ops;
    use crate::matrix::matrix::Matrix;

    // v: 2x2 vector to be rotated
    // angle: angle to rotate in radians; positive is counterclockwise
    pub fn rotate(v: &Vec<f64>, angle: f64) -> Result<Vec<f64>, String> {
        let m = Matrix::from(
        vec![
            vec![angle.cos(), angle.sin()],
            vec![-1.0 * angle.sin(), angle.cos()]
        ]);

        return Ok(m.vector_product(v).unwrap());
    }
}
