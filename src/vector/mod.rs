mod spec;

#[allow(dead_code)]
pub mod vector {
    // TODO: decide if this is possible...
    // implementation such that if this is true everything is done in terms of
    // Fractions rather than decimals. Thus all functions need to be typed so
    // that they return _either_ Vec<f64> or Vec<Fraction> depending. It would
    // be nice to not need to repeat too much logic though.
    pub static USE_FRACTIONS: bool = false;

    pub fn dot_product(a: &Vec<f64>, b: &Vec<f64>) -> Result<f64, String> {
        if a.len() != b.len() {
            return Err(String::from("vectors must have the same length to compute dot product"));
        }

        let mut product: f64 = 0.0;
        for i in 0..a.len() {
            product += a[i] * b[i];
        }

        return Ok(product);
    }

    pub fn projection(v: &Vec<f64>, w: &Vec<f64>) -> Result<Vec<f64>, String> {
        if v.len() != w.len() {
            return Err(String::from("vectors must have the same length to compute projection"));
        }

        let mut result: Vec<f64> = Vec::new();

        // we know these dot products won't fail
        let numerator = dot_product(v, w).unwrap();
        let denominator = dot_product(w, w).unwrap();

        for i in 0..v.len() {
            result.push((numerator / denominator) * w[i]);
        }

        return Ok(result);
    }

    pub fn projection_onto_basis(v: &Vec<f64>, basis: &Vec<Vec<f64>>) -> Result<Vec<f64>, String> {
        if !check_orthogonal_basis(basis) {
            return Err(String::from("projection requires and orthogonal basis"));
        }

        let mut result: Vec<f64> = vec![0.0, 0.0, 0.0];

        for w in basis {
            let p = projection(v, w).unwrap();
            println!("intermediate: {:?}", p);
            result = vector_addition(&result, &p).unwrap();
        }

        return Ok(result);
    }

    pub fn vector_addition(v: &Vec<f64>, w: &Vec<f64>) -> Result<Vec<f64>, String> {
        if v.len() != w.len() {
            return Err(String::from("Vectors must be the same length to compute sum"));
        }

        let mut result: Vec<f64> = Vec::new();
        for i in 0..v.len() {
            result.push(v[i] + w[i]);
        }

        return Ok(result);
    }

    pub fn vector_subtraction(v: &Vec<f64>, w: &Vec<f64>) -> Result<Vec<f64>, String> {
        if v.len() != w.len() {
            return Err(String::from("Vectors must be the same length to compute sum"));
        }

        let mut _w: Vec<f64> = Vec::new();
        for i in 0..w.len() {
            _w.push(w[i] * -1.0);
        }

        return vector_addition(v, &_w)
    }

    pub fn check_orthogonal_basis(basis: &Vec<Vec<f64>>) -> bool {
        for i in 0..basis.len() {
            for j in 0..basis.len() {
                if i != j {
                    if dot_product(&basis[i], &basis[j]).unwrap() != 0.0 {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn cross_product(v: &Vec<f64>, w: &Vec<f64>) -> Result<Vec<f64>, String> {
        if v.len() != 3 || w.len() != 3 {
            return Err(String::from("Cross product can only be computed for two 3-vectors"));
        }

        let e1 = v[1] * w[2] - v[2] * w[1];
        let e2 = v[2] * w[0] - v[0] * w[2];
        let e3 = v[0] * w[1] - v[1] * w[0];

        return Ok(vec![e1, e2, e3]);
    }

    pub fn scalar_product(v: &Vec<f64>, s: f64) -> Vec<f64> {
        let mut result = v.clone();
        for i in 0..v.len() {
            result[i] = v[i] * s;
        }
        return result;
    }

    pub fn add(v: &Vec<f64>, w: &Vec<f64>) -> Result<Vec<f64>, String> {
        if v.len() != w.len() {
            return Err(String::from("Vectors must be the same length to compute sum"));
        }

        let mut result = v.clone();
        for i in 0..w.len() {
            result[i] += w[i];
        }
        return Ok(result);
    }

    // TODO
    // pub fn linear_regression(_x: &Vec<f64>, _y: &Vec<f64>) {
        // let mut x = (*_x).clone();
        // let mut y = (*_y).clone();

        // let mut x_avg = 0;
        // for i in &x { x_avg += i; }
        // x_avg /= x.len() as f64;
        // for i in 0..x.len() { x[i] -= x_avg; }

        // let mut y_avg = 0;
        // for i in &y { y_avg += i; }
        // y_avg /= y.len() as f64;
        // for i in 0..y.len() { y[i] -= y_avg; }
    // }

}
