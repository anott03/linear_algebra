pub mod matrix {
    use std::array;

    #[derive(Debug)]
    pub struct Matrix {
        pub rows: i32,
        pub cols: i32,
        pub data: Vec<Vec<i32>>,
    }

    impl Matrix {
        pub fn new(n: i32, m: i32) -> Matrix {
            let data = vec![vec![0; m as usize]; n as usize];
            return Matrix {
                rows: n,
                cols: m,
                data,
            };
        }

        pub fn from(v: Vec<Vec<i32>>) -> Matrix {
            return Matrix {
                rows: v.len() as i32,
                cols: v[0].len() as i32,
                data: v,
            };
        }

        pub fn vector_product(v: Vec<i32>) -> Result<Vec<i32>, String> {
            let mut result: Vec<Vec<i32>> = Vec::new();
            for row in data {
                for entry in row {

                }
            }
        }
    }
}
