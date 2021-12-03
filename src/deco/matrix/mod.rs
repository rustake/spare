use std::fmt;

use palett::fluo::matrix::fluo_rendered;
use palett::types::Preset;
use veho::matrix::Mappers;
use veho::vector::mapper;

use crate::padder::matrix_padder;

pub fn deco_matrix<R, M>(mx: M,
                         de: Option<&str>,
                         presets: Option<(Preset, Preset)>,
) -> String where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    R::Item: fmt::Display
{
    let de = match de {
        None => ", ",
        Some(tx) => tx,
    };
    let texts = match presets {
        None => mx.mapper(|el| el.to_string()),
        Some(pr) => fluo_rendered(mx, &pr, &[]),
    };
    // let texts = fluo_rendered(mx, &(OCEAN, PLANET), &[]);
    let padded = matrix_padder(texts);
    let lines: Vec<String> = mapper(padded, |vec| format!("  [ {} ],", vec.join(de)));
    return format!("[\n{}\n]", lines.join("\n"));
}

// pub macro_rules! deco_matrix {
//     ($mx: expr) => {
//         deco_matrix($mx, ", ")
//     };
// }

#[cfg(test)]
mod deco_matrix_tests {
    use palett::presets::{METRO, OCEAN, PLANET, POME};
    use veho::matrix::{init, Utils};

    use crate::deco_matrix;

    #[test]
    fn test() {
        let matrix = init(5, 5, |i, j| ((i as i32) + 1) * 10_i32.pow(j as u32));
        let matrix = matrix.transpose();
        let texted = deco_matrix(&matrix, None, Some((OCEAN, PLANET)));
        println!("matrix = {}", texted);
        let texted = deco_matrix(&matrix, None, Some((POME, METRO)));
        println!("matrix = {}", texted);
    }
}