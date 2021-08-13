mod spec;

pub mod vector {
    pub fn dot_product(a: &Vec<i32>, b: &Vec<i32>) -> Result<i32, String> {
        if a.len() != b.len() {
            return Err(String::from("vectors must have the same length to compute dot product"));
        }

        let mut product = 0;
        for i in 0..a.len() {
            product += a[i] * b[i];
        }

        return Ok(product);
    }

    pub fn projection(v: &Vec<i32>, w: &Vec<i32>) -> Result<Vec<i32>, String> {
        if v.len() != w.len() {
            return Err(String::from("vectors must have the same length to compute projection"));
        }

        let mut result: Vec<i32> = Vec::new();

        let numerator = dot_product(v, w).unwrap();
        let denominator = dot_product(w, w).unwrap();

        for i in 0..v.len() {
            result.push((numerator / denominator) * w[i]);
        }

        return Ok(result);
    }

    pub fn projection_onto_basis(v: &Vec<i32>, basis: &Vec<Vec<i32>>) -> Result<Vec<i32>, String> {
        let mut result: Vec<i32> = vec![0, 0, 0];

        for i in 0..basis.len() {
            for j in 0..basis.len() {
                if i != j {
                    if dot_product(&basis[i], &basis[j]).unwrap() != 0 {
                        return Err(String::from("projection requires an orthogonal basis"));
                    }
                }
            }
        }

        for w in basis {
            let p = projection(v, w).unwrap();
            println!("intermediate: {:?}", p);
            result = vector_addition(&result, &p).unwrap();
        }

        return Ok(result);
    }

    pub fn vector_addition(v: &Vec<i32>, w: &Vec<i32>) -> Result<Vec<i32>, String> {
        if v.len() != w.len() {
            return Err(String::from("Vectors must be the same length to compute sum"));
        }

        let mut result: Vec<i32> = Vec::new();
        for i in 0..v.len() {
            result.push(v[i] + w[i]);
        }

        return Ok(result);
    }

    pub fn vector_subtraction(v: &Vec<i32>, w: &Vec<i32>) -> Result<Vec<i32>, String> {
        if v.len() != w.len() {
            return Err(String::from("Vectors must be the same length to compute sum"));
        }

        let mut _w: Vec<i32> = Vec::new();
        for i in 0..w.len() {
            _w.push(w[i] * -1);
        }

        return vector_addition(v, &_w)
    }
}
