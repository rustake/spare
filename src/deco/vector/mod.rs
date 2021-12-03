use std::fmt::Display;

use palett::fluo::vector::fluo_rendered;
use palett::types::Preset;
use veho::vector::Mappers;

mod joiners;

trait Decorable: IntoIterator where
    Self: Sized,
    Self::Item: Display {
    fn deco_vector(self,
                   de: Option<&str>,
                   presets: Option<&(Preset, Preset)>, ) -> String
    {
        let delim = match de { None => ", ", Some(tx) => tx, };
        let texts = match presets {
            None => self.mapper(|x| x.to_string()),
            Some(p) => fluo_rendered(self, p, &[]),
        };
        return format!("[ {} ]", texts.join(delim));
    }
}

impl<I> Decorable for I where
    I: IntoIterator,
    I::Item: Display
{}


// #[cfg(feature = "use_alloc")]
pub fn deco_vector<I>(vec: I,
                      de: Option<&str>,
                      presets: Option<&(Preset, Preset)>, ) -> String where
    I: IntoIterator,
    I::Item: Display
{ vec.deco_vector(de, presets) }

#[cfg(test)]
mod tests {
    use palett::presets::{FRESH, OCEAN};

    use crate::deco_vector;

    #[test]
    fn test() {
        let presets = (OCEAN, FRESH);
        let words = vec!["a", "b", "c", "-", "1", "2", "3"];
        words.join(", ");
        let rendered = deco_vector(&words, Some(", "), Some(&presets));
        println!(">> [rendered] {}", rendered);
        println!(">> [len] {}", words.len());
        println!(">> [capacity] {}", words.capacity());
        let rendered = deco_vector(&words, Some(", "), Some(&presets));
        println!(">> [rendered] {}", rendered);
    }
}