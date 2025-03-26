use std::cmp::Ordering;

pub struct Keyed<T, K>(T, K);

impl<T, K> Keyed<T, K> {
    pub fn new(value: T, key: K) -> Self {
        Keyed(value, key)
    }
    
    pub fn value(&self) -> &T {
        &self.0
    }
    
    pub fn key(&self) -> &K {
        &self.1
    }
}

pub trait ToKeyed : Sized {
    fn with_key<K>(self, key: K) -> Keyed<Self, K>;
}

impl<T: Sized> ToKeyed for T {
    fn with_key<K>(self, key: K) -> Keyed<Self, K> {
        Keyed(self, key)
    }
}

impl<T: Sized, K: PartialEq> PartialEq for Keyed<T, K> {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl<T: Sized, K: Eq> Eq for Keyed<T, K> {}

impl<T: Sized, K: PartialOrd> PartialOrd for Keyed<T, K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl<T: Sized, K: Ord> Ord for Keyed<T, K> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl<T: Sized + Clone, K: Clone> Clone for Keyed<T, K> {
    fn clone(&self) -> Self {
        Keyed(self.0.clone(), self.1.clone())
    }
}

impl<T: Sized + Copy, K: Copy> Copy for Keyed<T, K> {}

impl<T: Sized + std::fmt::Debug, K: std::fmt::Debug> std::fmt::Debug for Keyed<T, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Keyed")
           .field("value", &self.0)
           .field("key", &self.1)
           .finish()
    }
}

impl<T: Sized + std::fmt::Display, K: std::fmt::Display> std::fmt::Display for Keyed<T, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({})", self.0, self.1)
    }
}

impl<T: Sized + std::hash::Hash, K: std::hash::Hash> std::hash::Hash for Keyed<T, K> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}