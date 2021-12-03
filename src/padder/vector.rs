use aryth::indicator::vector::Indicators;
use veho::vector::mapper as map_vector;

use crate::padder::util::len_selector;

pub fn vector_padder(vec: Vec<String>, ansi: bool) -> Vec<String>
{
    let indicator = len_selector(ansi);
    let width = (&vec).max_by(indicator).unwrap_or(0);
    return map_vector(vec, |x| format!("{: >w$}", x, w = width));
}

#[cfg(test)]
mod tests {
    use num::pow;
    use veho::vector::init;

    use super::*;

    #[test]
    fn test_vector_padder() {
        let vec = init(12, |i| pow(2, i).to_string());
        let padded = vector_padder(vec, true);
        println!(">> [padded] {:?}", padded);
        // println!(">> [vec] {:?}", &vec);
    }
}


