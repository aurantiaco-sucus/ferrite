use std::collections::BTreeMap;
use std::ops::Index;
use crate::{VecApplyOrder, VecArgSort};

pub fn order_with<T: Ord + ?Sized, U>(keys: &[&T], mut values: Vec<U>) -> Vec<U> {
    let order = keys.arg_sort();
    values.apply_order(order);
    values
}

pub fn order_map<'a, T: Ord + ?Sized>(keys: impl IntoIterator<Item=&'a T>) -> BTreeMap<&'a T, usize> {
    let keys = keys.into_iter().collect::<Vec<_>>();
    let order = keys.arg_sort();
    keys.into_iter().zip(order).collect()
}

pub struct OrderMap<'a, T: Ord + ?Sized>(BTreeMap<&'a T, usize>);

impl<'a, T: Ord + ?Sized> OrderMap<'a, T> {
    pub fn new(keys: impl IntoIterator<Item=&'a T>) -> Self {
        let keys = keys.into_iter().collect::<Vec<_>>();
        let order = keys.arg_sort();
        Self(keys.into_iter().zip(order).collect())
    }

    pub fn get(&self, key: &T) -> Option<usize> {
        self.0.get(key).copied()
    }

    pub fn reorder<U>(&self, values: Vec<(&'a T, U)>) -> Vec<U> {
        let order = values.iter().map(|(k, _)| self.0[k]).collect::<Vec<_>>();
        let mut values = values.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
        values.apply_order(order);
        values
    }
}

impl<'a, T: Ord + ?Sized> Index<&'a T> for OrderMap<'a, T> {
    type Output = usize;

    fn index(&self, key: &'a T) -> &Self::Output {
        self.0.get(key).unwrap()
    }
}