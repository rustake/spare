use aryth::indicator::vector::Indicators;
use veho::columns::Mappers as ColumnsMapper;
use veho::matrix::{Mappers, Matrix};

use crate::padder::util::len_selector;

pub fn matrix_padder(mx: Matrix<String>, ansi: bool) -> Matrix<String> where
{
    let indicator = len_selector(ansi);
    let widths = (&mx).map_columns(|col| (&col).max_by(indicator).unwrap_or(0));
    return mx.indexed_mapper(|_, j, x| format!("{: >w$}", x, w = widths[j]));
}


#[cfg(test)]
mod tests {
    use veho::matrix::init;

    use super::*;

    #[test]
    fn test_matrix_padder() {
        let matrix = init(4, 3, |i, j| (2 ^ (i + 1) * 2 + (j + 1)).to_string());
        println!("{:?}", matrix_padder(matrix, true));
        // println!("{:?}", matrix_padder(matrix, true));
    }
}