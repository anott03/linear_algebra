mod spec;

#[allow(dead_code)]
pub mod transformation {
    use std::ops;
    use crate::matrix::matrix::Matrix;

    // v: 2x2 vector to be rotated
    // angle: angle to rotate in radians; positive is counterclockwise
    pub fn rotate(v: &Vec<i32>, angle: f64) -> Result<Vec<i32>, String> {

        let m = Matrix::from(vec![vec![angle.cos(), -1 * angle.sin()],
            vec![angle.sin(), angle.cos()]]);

        return Ok(Vec::new());
    }
}
