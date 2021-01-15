use std::fmt;
use std::fmt::Write;

use palett::fluo::vector::fluo_rendered;
use palett::presets::{FRESH, OCEAN};
use veho::vector::Mappers;

pub trait Itertools: Iterator {
    fn join(&mut self, sep: &str) -> String
        where Self::Item: fmt::Display
    {
        match self.next() {
            None => String::new(),
            Some(first_elt) => {
                // estimate lower bound of capacity needed
                let (lower, _) = self.size_hint();
                let mut result = String::with_capacity(sep.len() * lower);
                write!(&mut result, "{}", first_elt).unwrap();
                self.for_each(|elt| {
                    result.push_str(sep);
                    write!(&mut result, "{}", elt).unwrap();
                });
                result
            }
        }
    }
}

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
pub fn deco_vector<I>(it: I, de: &str) -> String where
    I: IntoIterator,
    I::Item: fmt::Display
{
    it.deco(de)
}

#[cfg(test)]
mod tests {
    use crate::deco_vector;

    #[test]
    fn test() {
        let words = vec!["a", "b", "c", "-", "1", "2", "3"];
        words.join(", ");
        let rendered = deco_vector(&words, ", ");
        println!("{}", rendered);
    }
}