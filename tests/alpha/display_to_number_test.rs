use std::fmt::{Display, Formatter};
use std::fmt;
use std::num::ParseFloatError;

use veho::vector::Mappers;

pub enum Value {
    Num(f32),
    Str(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Num(n) => { write!(f, "{}", n) }
            Value::Str(t) => { write!(f, "{}", t) }
        }
    }
}

fn display_vec_to_value_vec<T: Display>(vec: Vec<T>) -> Vec<Value> {
    vec.mapper(|x| {
        let stringed = x.to_string();
        return match stringed.parse::<f32>() {
            Ok(v) => { Value::Num(v) }
            Err(_) => { Value::Str(stringed) }
        };
    })
}

#[cfg(test)]
mod tests {
    use spare::deco_vector;

    use super::*;

    #[test]
    fn display_to_number_test() {
        let vec = vec!["1", "foo", "bar"];
        let result = display_vec_to_value_vec(vec);
        println!("{}", deco_vector(result, ", "));
    }
}