use std::fmt::Display;

use palett::fluo::matrix::fluo_rendered;
use palett::types::Preset;
use veho::matrix::Mappers;
use veho::vector::mapper;

use crate::padder::matrix_padder;

pub trait Decorable<R>: IntoIterator<Item=R> where
    R: IntoIterator,
    R::Item: Display,
    Self: Sized,
{
    fn deco_matrix(self, de: Option<&str>, presets: Option<&(Preset, Preset)>) -> String {
        let delim = match de {
            None => ", ",
            Some(tx) => tx,
        };
        let texts = match presets {
            None => self.mapper(|x| x.to_string()),
            Some(p) => fluo_rendered(self, p, &[]),
        };
        let padded = matrix_padder(texts, true);
        let lines: Vec<String> = mapper(padded, |vec| format!("  [ {} ],", vec.join(delim)));
        return format!("[\n{}\n]", lines.join("\n"));
    }
}

impl<M, R> Decorable<R> for M where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    R::Item: Display
{}

pub fn deco_matrix<R, M>(mx: M,
                         de: Option<&str>,
                         presets: Option<&(Preset, Preset)>, ) -> String where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    R::Item: Display
{
    mx.deco_matrix(de, presets)
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
        let texted = deco_matrix(&matrix, None, Some(&(OCEAN, PLANET)));
        println!(">> matrix = {}", texted);
        let texted = deco_matrix(&matrix, None, Some(&(POME, METRO)));
        println!(">> matrix = {}", texted);
    }
}