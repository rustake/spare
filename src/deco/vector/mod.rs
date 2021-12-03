use std::fmt;
use std::fmt::Write;

use palett::fluo::vector::fluo_rendered;
use palett::presets::{FRESH, OCEAN};
use veho::vector::Mappers;

pub trait Itertools: Iterator {
    fn join(&mut self, delim: &str) -> String
        where Self::Item: fmt::Display
    {
        match self.next() {
            None => String::new(),
            Some(first) => {
                // estimate lower bound of capacity needed
                let (lower, _) = self.size_hint();
                let mut result = String::with_capacity(delim.len() * lower);
                write!(&mut result, "{}", first).unwrap();
                self.for_each(|element| {
                    result.push_str(delim);
                    write!(&mut result, "{}", element).unwrap();
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