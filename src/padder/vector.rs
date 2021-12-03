use aryth::indicator::vector::Indicators;
use veho::vector::mapper as map_vector;

use crate::padder::util::{len_selector, pad_left};

pub fn vector_padder(vec: Vec<String>, ansi: bool) -> Vec<String>
{
    let indicator = len_selector(ansi);
    let width = (&vec).max_by(indicator).unwrap_or(0);
    return map_vector(vec, |x| pad_left(&x, width, ansi));
}

#[cfg(test)]
mod tests {
    use num::pow;
    use veho::vector::init;

    use super::*;

    const CSI: &str = "\x1b[";
    const SGR: &str = "m";
    const RESET: &str = "\x1b[0m";

    #[test]
    fn test_vector_padder() {
        let vec = init(12, |i| pow(2, i).to_string());
        let vec = init(12, |i| format!("{}{}{}{}{}{}{}", CSI, "38;2;", i * 6 + 127, ";12;12", SGR, pow(2, i), RESET));
        let padded = vector_padder(vec, true);
        println!(">> [padded] \n{:}", padded.join("\n"));
        // println!(">> [vec] {:?}", &vec);
    }
}


