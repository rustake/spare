use aryth::indicator::vector::Indicators;
use texting::lange;

pub fn get_len<T: AsRef<str>>(s: &T) -> usize
{ s.as_ref().len() }

pub fn get_ansi_len<T: AsRef<str>>(s: &T) -> usize
{ lange(s.as_ref()) }

pub fn len_selector<T: AsRef<str>>(ansi: bool) -> fn(&T) -> usize
{ if ansi { get_ansi_len } else { get_len } }

pub fn max_len<T: AsRef<str>>(col: &Vec<T>, ansi: bool) -> usize
{
    let indicator = len_selector(ansi);
    let max = (&col).max_by(indicator).unwrap_or(0);
    max
}

pub fn ansi_pad_len<T: AsRef<str>>(tx: &T, pd: usize) -> usize {
    let value = tx.as_ref();
    // if value.HasAnsi() { pd } else {  }
    value.len() + pd - lange(value)
}

pub fn pad_left<T: AsRef<str>>(tx: &T, width: usize, ansi: bool) -> String {
    if ansi {
        format!("{: >w$}", tx.as_ref(), w = ansi_pad_len(tx, width))
    } else {
        format!("{: >w$}", tx.as_ref(), w = width)
    }
}

#[cfg(test)]
mod tests {
    use num::pow;
    use texting::Concat;
    use veho::vector::init;

    use super::*;

    const CSI: &str = "\x1b[";
    const SGR: &str = "m";
    const RESET: &str = "\x1b[0m";

    #[test]
    fn test_vector_util() {
        let vec = init(12, |i| format!("{}{}{}{}{}{}{}", CSI, "38;2;", i * 6 + 127, ";12;12", SGR, pow(2, i), RESET));
        println!(">> [max_len] {:}", max_len(&vec, true));
        println!(">> [vec] {:}", &vec.join(", "));
    }
}