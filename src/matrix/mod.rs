#[allow(dead_code)]
pub mod matrix {
    use crate::vector::vector;

    #[derive(Debug)]
    pub struct Matrix {
        pub rows: f64,
        pub cols: f64,
        pub data: Vec<Vec<f64>>,
    }

    impl Matrix {
        pub fn new(n: f64, m: f64) -> Matrix {
            let data = vec![vec![0.0; m as usize]; n as usize];
            return Matrix {
                rows: n,
                cols: m,
                data,
            };
        }

        pub fn from(v: Vec<Vec<f64>>) -> Matrix {
            return Matrix {
                rows: v.len() as f64,
                cols: v[0].len() as f64,
                data: v,
            };
        }

        pub fn vector_product(&self, v: &Vec<f64>) -> Result<Vec<f64>, String> {
            let mut result: Vec<f64> = Vec::new();
            for i in 0..self.data.len() {
                for j in 0..v.len() {
                    result = vector::add(&result, &vector::scalar_product(&self.data[i], v[j])).unwrap();
                }
            }
            return Ok(result);
        }
    }
}
