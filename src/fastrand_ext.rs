use std::collections::BTreeMap;

pub use fastrand;

pub trait FastrandMapExt<K, V> {
    fn insert_randomized(&mut self, value: V) -> K;
}

impl<V> FastrandMapExt<u64, V> for BTreeMap<u64, V> {
    fn insert_randomized(&mut self, value: V) -> u64 {
        let mut key = fastrand::u64(0..u64::MAX);
        while self.contains_key(&key) {
            key = fastrand::u64(0..u64::MAX);
        }
        self.insert(key, value);
        key
    }
}