use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use compacts::bits;

pub struct Table<K: Hash + Eq = String, V: Ord = String> {
    kvs: HashMap<K, BTreeMap<V, bits::Set>>,
}

impl<K: Hash + Eq, V: Ord> Table<K, V> {
    pub fn new() -> Table<K, V> {
        Table { kvs: HashMap::new() }
    }

    pub fn set_key_value(&mut self, key: K, val: V, bit: bool) {
        let values = self.kvs.entry(key).or_insert_with(BTreeMap::new);
        let bitmap = values.entry(val).or_insert_with(bits::Set::new);
        bitmap.insert(if bit { 1 } else { 0 });
    }

    // pub fn get_key_value(&mut self, key: K, val: V) -> bool {
    //     // TODO use mutable instead
    //     let values = self.kvs.entry(key).or_insert_with(BTreeMap::new);
    //     let bitmap = values.entry(val).or_insert_with(bits::Set::new);
    // }
}

#[test]
fn new_table() {
    let mut new_table: Table<String, String> = Table::new();
    new_table.set_key_value("hello".to_string(), "world".to_string(), true)
}
