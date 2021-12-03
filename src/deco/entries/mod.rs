use std::fmt;

use palett::fluo::entries::fluo_rendered;
use palett::presets::{OCEAN, PLANET};
use veho::vector::mapper;

pub fn deco_entries<K, V, KVS>(entries: KVS, de: &str) -> String where
    K: fmt::Display,
    V: fmt::Display,
    KVS: IntoIterator<Item=(K, V)>
{
    let texts = fluo_rendered(entries, &(OCEAN, PLANET), &[]);
    let lines: Vec<String> = mapper(texts, |(k, v)| format!("  {}{}{},", k, de, v));
    return format!("{{\n{}\n}}", lines.join("\n"));
}

#[cfg(test)]
mod deco_entries_tests {
    use veho::entries::IntoHashmap;

    use crate::deco_entries;

    #[test]
    fn test() {
        let entries = vec![
            ("un", "1"),
            ("deux", "2"),
            ("trois", "3"),
            ("quatre", "4"),
            ("cinq", "5"),
            ("six", "6"),
        ].into_hashmap();
        let texted_mx = deco_entries(entries, ": ");
        println!("entries = {}", texted_mx);
    }
}