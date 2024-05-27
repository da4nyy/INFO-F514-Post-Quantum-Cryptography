use std::ops::Mul;


trait MatrixElement: Mul<Output = Self> + Copy {
    fn to_u32(self) -> u32;
}
impl MatrixElement for u16 {
    fn to_u32(self) -> u32 {
        self as u32
    }
}
impl MatrixElement for u32 {
    fn to_u32(self) -> u32 {
        self
    }
}

/**
 * Multiply a 2D matrix with a 3D matrix 
 */
pub fn matrix_multiply<T>(a: &Vec<Vec<u8>>, b: &Vec<Vec<Vec<T>>>) -> Vec<Vec<Vec<u32>>> where T: MatrixElement, {
    let mut result = vec![vec![vec![0u32; b[0][0].len()]; b[0].len()]; a.len()];
    
    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..a[0].len() {
                for l in 0..b[0][0].len() {
                    result[i][j][l] += (a[i][k] as u32) * (b[k][j][l].to_u32());
                }
            }
        }
    }

    result
}

/**
 * Negate a 3D matrix 
 */
pub fn element_wise_negation(matrix: &Vec<Vec<Vec<u32>>>) -> Vec<Vec<Vec<i32>>> {
    let mut result = vec![vec![vec![0; matrix[0][0].len()]; matrix[0].len()]; matrix.len()];
    
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            for k in 0..matrix[i][j].len() {
                result[i][j][k] = -(matrix[i][j][k] as i32);
            }
        }
    }

    result
}

/**
 * Substracte two 3D matrix
 */
pub fn element_wise_subtraction(a: &Vec<Vec<Vec<i32>>>, b: &Vec<Vec<Vec<u32>>>) -> Vec<Vec<Vec<i32>>> {
    let mut result = vec![vec![vec![0; a[0][0].len()]; a[0].len()]; a.len()];
    
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            for k in 0..a[i][j].len() {
                result[i][j][k] = a[i][j][k] - b[i][j][k] as i32;
            }
        }
    }

    result
}

/**
 * Transpose a 2D matrix (ex: 3*2 -> 2*3)
 */
pub fn transpose(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut transposed = vec![vec![0; matrix.len()]; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}

pub fn upper(p: &Vec<Vec<f64>>, rows: usize) -> Vec<Vec<f64>> {
    let n = p.len();
    let mut result = vec![vec![p[0][0]; n]; n];

    for i in 0..rows {
        for j in 0..rows {
            if i < j {
                result[i][j] = p[i][j] + p[j][i];
            }
        }
    }

    result
}

pub fn pad_matrix(matrix: &Vec<Vec<Vec<u32>>>, d1: usize,d2: usize,d3: usize) -> Vec<Vec<Vec<u32>>> {
    let mut padded_matrix = vec![vec![vec![0; d3]; d2]; d1];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            for k in 0..matrix[i][j].len() {
                padded_matrix[i][j][k] = matrix[i][j][k];
            }
        }
    }

    padded_matrix
}