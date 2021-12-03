use std::fmt::{Display, Write};

pub trait Joiners: Iterator where
    Self::Item: Display
{
    fn join(&mut self, delim: &str) -> String
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