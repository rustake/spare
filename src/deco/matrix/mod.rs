use std::fmt;

use palett::fluo::matrix::fluo_rendered;
use palett::presets::{OCEAN, PLANET};
use veho::vector::mapper;

pub fn deco_matrix<R, M>(mx: M, de: &str) -> String where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
    R::Item: fmt::Display
{
    let texts = fluo_rendered(mx, &(OCEAN, PLANET), &[]);
    let lines: Vec<String> = mapper(texts, |vec| format!("  [ {} ],", vec.join(de)));
    return format!("[\n{}\n]", lines.join("\n"));
}

#[cfg(test)]
mod deco_matrix_tests {
    use veho::matrix::init;

    use crate::deco_matrix;

    #[test]
    fn test() {
        let mx = init(3, 5, |i, j| (i + j) as i32);
        let texted_mx = deco_matrix(mx, ", ");
        println!("matrix = {}", texted_mx);
    }
}