use aryth::indicator::vector::Indicators;
use veho::columns::mapper as map_columns;
use veho::matrix::{Mappers, Matrix};

pub fn matrix_padder(mx: Matrix<String>) -> Matrix<String> where
{
    let widths = map_columns(&mx, |col| col.max_by(|x| x.len()).unwrap_or(0));
    mx.indexed_mapper(|_, j, x| format!("{: >w$}", x, w = widths[j]))
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