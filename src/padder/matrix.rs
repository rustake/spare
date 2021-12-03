use aryth::indicator::vector::Indicators;
use veho::columns::mapper as map_columns;
use veho::matrix::{Mappers, Matrix};
use veho::vector::mapper as map_vector;

pub fn matrix_padder(mx: Matrix<String>) -> Matrix<String>
{
    let widths = map_columns(&mx, |col| col.max_by(|x| x.len()).unwrap_or(0));
    return mx.indexed_mapper(|_, j, x| format!("{: >w$}", x, w = widths[j]));
}

pub fn vector_padder(vec: Vec<String>) -> Vec<String>
{
    let width = (&vec).max_by(|x| x.len()).unwrap_or(0);
    return map_vector(vec, |x| format!("{: >w$}", x, w = width));
}

#[cfg(test)]
mod tests {
    use veho::matrix::init;

    use super::*;

    #[test]
    fn test_matrix_padder() {
        let matrix = init(4, 3, |i, j| (2 ^ (i + 1) * 2 + (j + 1)).to_string());
        println!("{:?}", matrix_padder(matrix));
    }
}