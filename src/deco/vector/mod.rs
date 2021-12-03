use std::fmt;

use palett::fluo::vector::fluo_rendered;
use palett::presets::{FRESH, OCEAN};
use palett::types::Preset;
use veho::vector::Mappers;

mod joiners;

trait Decorable: IntoIterator {
    fn deco(self, de: &str) -> String where
        Self: Sized,
        Self::Item: fmt::Display
    {
        let texts = self.mapper(|x| format!("{}", x));
        return format!("[ {} ]", texts.join(de));
    }
}

impl<I> Decorable for I where
    I: IntoIterator,
    I::Item: fmt::Display {
    fn deco(self, de: &str) -> String {
        let texts = fluo_rendered(self, &(OCEAN, FRESH), &[]);
        return format!("[ {} ]", texts.join(de));
    }
}


// #[cfg(feature = "use_alloc")]
pub fn deco_vector<I>(vec: I,
                      de: Option<&str>,
                      presets: Option<&(Preset, Preset)>, ) -> String where
    I: IntoIterator,
    I::Item: fmt::Display
{
    let de = match de { None => ", ", Some(tx) => tx, };
    let texts = match presets {
        None => vec.mapper(|x| x.to_string()),
        Some(p) => fluo_rendered(vec, p, &[]),
    };
    return format!("[ {} ]", texts.join(de));
    vec.deco(de)
}

#[cfg(test)]
mod tests {
    use palett::presets::{FRESH, OCEAN};

    use crate::deco_vector;

    #[test]
    fn test() {
        let words = vec!["a", "b", "c", "-", "1", "2", "3"];
        words.join(", ");
        let rendered = deco_vector(&words, Some(", "), Some(&(OCEAN, FRESH)));
        println!(">> [vector] {}", rendered);
        println!(">> [len] {}", words.len());
        println!(">> [capacity] {}", words.capacity())
    }
}