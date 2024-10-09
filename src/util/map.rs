use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

pub trait InsertHelper<K, V> {
    fn insert_if_absent(&mut self, key: K, value: V);
}

impl<K: Eq + Hash, V: Hash> InsertHelper<K, V> for HashMap<K, V> {
    fn insert_if_absent(&mut self, key: K, value: V) {
        if !self.contains_key(&key) {
            self.insert(key, value);
        }
    }
}

impl<K: Ord, V> InsertHelper<K, V> for BTreeMap<K, V> {
    fn insert_if_absent(&mut self, key: K, value: V) {
        if !self.contains_key(&key) {
            self.insert(key, value);
        }
    }
}
