use std::fmt::Display;

use palett::fluo::vector::fluo_rendered;
use palett::types::Preset;
use veho::entries::unwind;
use veho::vector::{Mappers, zipper};

use crate::padder::vector_padder;

pub trait Decorable<K, V>: IntoIterator<Item=(K, V)> where
    K: Display,
    V: Display,
    Self: Sized
{
    fn deco_entries(self,
                    de: Option<&str>,
                    presets: Option<&(Preset, Preset)>, ) -> String where
    {
        let delim = match de { None => ": ", Some(tx) => tx, };
        let (keys, items) = unwind(self);
        let (keys, items) = match presets {
            None => (
                keys.mapper(|x| x.to_string()),
                items.mapper(|x| x.to_string())
            ),
            Some(p) => (
                fluo_rendered(keys, p, &[]),
                fluo_rendered(items, p, &[])
            ),
        };
        let (keys, items) = (
            vector_padder(keys, true),
            vector_padder(items, true)
        );

        let lines: Vec<String> = zipper(keys, items, |k, v| format!("  {}{}{},", k, delim, v));
        return format!("{{\n{}\n}}", lines.join("\n"));
    }
}

impl<K, V, KVS> Decorable<K, V> for KVS where
    K: Display,
    V: Display,
    KVS: IntoIterator<Item=(K, V)>
{}

pub fn deco_entries<K, V, KVS>(entries: KVS,
                               de: Option<&str>,
                               presets: Option<&(Preset, Preset)>, ) -> String where
    K: Display,
    V: Display,
    KVS: IntoIterator<Item=(K, V)>
{ entries.deco_entries(de, presets) }

#[cfg(test)]
mod deco_entries_tests {
    use palett::presets::{METRO, OCEAN};
    use veho::entries::IntoHashmap;

    use crate::deco::entries::Decorable;

// use crate::deco_entries;

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
        let texted_mx = (&entries).deco_entries(Some(": "), Some(&(OCEAN, METRO)));
        println!(">> [entries] {}", texted_mx);
        println!(">> [len] {}", entries.len());
        println!(">> [capacity] {}", entries.capacity());
    }
}