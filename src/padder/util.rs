use aryth::indicator::vector::Indicators;
use texting::lange;

pub fn get_len<T: AsRef<str>>(s: &T) -> usize
{ s.as_ref().len() }

pub fn get_ansi_len<T: AsRef<str>>(s: &T) -> usize
{ lange(s.as_ref()) }

pub fn len_selector<T: AsRef<str>>(ansi: bool) -> fn(&T) -> usize
{ if ansi { get_len } else { get_ansi_len } }

pub fn max_len<T: AsRef<str>>(col: &Vec<T>, ansi: bool) -> usize
{
    let indicator = len_selector(ansi);
    let max = (&col).max_by(indicator).unwrap_or(0);
    max
}

#[cfg(test)]
mod tests {
    use num::pow;
    use veho::vector::init;

    use super::*;

    #[test]
    fn test_vector_util() {
        let vec = init(12, |i| pow(2, i).to_string());

        println!(">> [max_len] {:}", max_len(&vec, true));
        println!(">> [vec] {:?}", &vec);
    }
}