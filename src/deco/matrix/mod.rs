use std::fmt;

use palett::fluo::matrix::fluo_rendered;
use palett::presets::{OCEAN, PLANET};
use veho::vector::mapper;

use crate::padder::matrix_padder;

pub fn deco_matrix<R, M>(mx: M, de: &str) -> String where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
    R::Item: fmt::Display
{
    let texts = fluo_rendered(mx, &(OCEAN, PLANET), &[]);
    let padded = matrix_padder(texts);
    let lines: Vec<String> = mapper(padded, |vec| format!("  [ {} ],", vec.join(de)));
    return format!("[\n{}\n]", lines.join("\n"));
}

#[cfg(test)]
mod deco_matrix_tests {
    use veho::matrix::init;

    use crate::deco_matrix;

    #[test]
    fn test() {
        let matrix = init(5, 5, |i, j| ((i as i32) + 1) * 10_i32.pow(j as u32));
        let texted = deco_matrix(matrix, ", ");
        println!("matrix = {}", texted);
    }
}