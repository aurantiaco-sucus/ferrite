use std::collections::BTreeMap;
use std::io;
use std::ops::Index;

pub struct Bitmap {
    data: Vec<u128>,
    len: usize,
}

impl Bitmap {
    pub fn new(len: usize) -> Self {
        let mut data = vec![0; len.div_ceil(128)];
        if len % 128 != 0 {
            data[len.div_ceil(128) - 1] = !0 << (len % 128);
        }
        Self { data, len }
    }

    pub fn new_filled(len: usize) -> Self {
        let data = vec![!0; len.div_ceil(128)];
        Self { data, len }
    }

    pub fn get(&self, index: usize) -> bool {
        self.data[index / 128] & (1 << (index % 128)) != 0
    }

    pub fn set(&mut self, index: usize, value: bool) {
        let mask = 1 << (index % 128);
        if value {
            self.data[index / 128] |= mask;
        } else {
            self.data[index / 128] &= !mask;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn filled(&self) -> bool {
        self.data.iter().all(|&x| x == !0)
    }

    pub fn and(&self, other: &Self) -> Self {
        assert_eq!(self.len, other.len);
        let data = self.data.iter().zip(&other.data).map(|(&a, &b)| a & b).collect();
        Self { data, len: self.len }
    }

    pub fn or(&self, other: &Self) -> Self {
        assert_eq!(self.len, other.len);
        let data = self.data.iter().zip(&other.data).map(|(&a, &b)| a | b).collect();
        Self { data, len: self.len }
    }

    pub fn in_place_and(&mut self, other: &Self) {
        assert_eq!(self.len, other.len);
        for (a, &b) in self.data.iter_mut().zip(&other.data) {
            *a &= b;
        }
    }

    pub fn in_place_or(&mut self, other: &Self) {
        assert_eq!(self.len, other.len);
        for (a, &b) in self.data.iter_mut().zip(&other.data) {
            *a |= b;
        }
    }
    
    pub fn inverse(&self) -> Self {
        let mut data: Vec<u128> = self.data.iter().map(|&x| !x).collect();
        if self.len % 128 != 0 {
            data[self.len.div_ceil(128) - 1] &= !(!0 << (self.len % 128));
        }
        Self { data, len: self.len }
    }
    
    pub fn iter(&self) -> BitmapIter {
        BitmapIter { bitmap: self, current: 0 }
    }
}

pub struct BitmapIter<'a> {
    bitmap: &'a Bitmap,
    current: usize
}

impl Iterator for BitmapIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.bitmap.len() {
            Some(self.bitmap.get(self.current))
        } else {
            None
        }
    }
}

pub trait VecArgSort {
    fn arg_sort(&self) -> Vec<usize>;
}

impl<T> VecArgSort for [T] where T: Ord {
    fn arg_sort(&self) -> Vec<usize> {
        let mut indices = (0..self.len()).collect::<Vec<_>>();
        indices.sort_by_key(|&i| &self[i]);
        indices
    }
}

pub trait VecApplyOrder {
    fn apply_order(&mut self, order: Vec<usize>);
}

impl<T> VecApplyOrder for [T] {
    fn apply_order(&mut self, mut order: Vec<usize>) {
        assert_eq!(self.len(), order.len());
        while !order.is_sorted() {
            for i in 0..order.len() {
                if i == order[i] {
                    continue
                }
                let ti = order[i];
                self.swap(i, ti);
                order.swap(i, ti);
            }
        }
    }
}

pub fn read_be_u16(slice: &[u8]) -> u16 {
    let mut bytes = [0u8; 2];
    bytes.copy_from_slice(slice);
    u16::from_be_bytes(bytes)
}

pub fn read_be_u32(slice: &[u8]) -> u32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(slice);
    u32::from_be_bytes(bytes)
}

pub fn read_be_u64(slice: &[u8]) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(slice);
    u64::from_be_bytes(bytes)
}

pub fn read_be_i64(slice: &[u8]) -> i64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(slice);
    i64::from_be_bytes(bytes)
}

pub fn read_be_f64(slice: &[u8]) -> f64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(slice);
    f64::from_be_bytes(bytes)
}

pub trait WriteExt {
    fn write_u8(&mut self, value: u8) -> io::Result<()>;
    fn write_u16(&mut self, value: u16) -> io::Result<()>;
    fn write_u32(&mut self, value: u32) -> io::Result<()>;
    fn write_u64(&mut self, value: u64) -> io::Result<()>;
    fn write_i64(&mut self, value: i64) -> io::Result<()>;
    fn write_f64(&mut self, value: f64) -> io::Result<()>;
    fn write_string_len16(&mut self, value: &str) -> io::Result<()>;
    fn write_string_len32(&mut self, value: &str) -> io::Result<()>;
}

impl<W: io::Write> WriteExt for W {
    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.write_all(&[value])
    }

    fn write_u16(&mut self, value: u16) -> io::Result<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_u32(&mut self, value: u32) -> io::Result<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_u64(&mut self, value: u64) -> io::Result<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_i64(&mut self, value: i64) -> io::Result<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_f64(&mut self, value: f64) -> io::Result<()> {
        self.write_all(&value.to_be_bytes())
    }

    fn write_string_len16(&mut self, value: &str) -> io::Result<()> {
        let bytes = value.as_bytes();
        if bytes.len() > u16::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "String too long"));
        }
        self.write_u16(bytes.len() as u16)?;
        self.write_all(bytes)
    }

    fn write_string_len32(&mut self, value: &str) -> io::Result<()> {
        let bytes = value.as_bytes();
        if bytes.len() > u32::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "String too long"));
        }
        self.write_u32(bytes.len() as u32)?;
        self.write_all(bytes)
    }
}

pub trait ReadExt {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_u32(&mut self) -> io::Result<u32>;
    fn read_u64(&mut self) -> io::Result<u64>;
    fn read_i64(&mut self) -> io::Result<i64>;
    fn read_f64(&mut self) -> io::Result<f64>;
    fn read_string_len16(&mut self) -> io::Result<String>;
    fn read_string_len32(&mut self) -> io::Result<String>;
}

impl<R: io::Read> ReadExt for R {
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0u8; 1];
        self.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        let mut buffer = [0u8; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    fn read_u64(&mut self) -> io::Result<u64> {
        let mut buffer = [0u8; 8];
        self.read_exact(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    fn read_i64(&mut self) -> io::Result<i64> {
        let mut buffer = [0u8; 8];
        self.read_exact(&mut buffer)?;
        Ok(i64::from_be_bytes(buffer))
    }

    fn read_f64(&mut self) -> io::Result<f64> {
        let mut buffer = [0u8; 8];
        self.read_exact(&mut buffer)?;
        Ok(f64::from_be_bytes(buffer))
    }

    fn read_string_len16(&mut self) -> io::Result<String> {
        let len = self.read_u16()? as usize;
        let mut buffer = vec![0u8; len];
        self.read_exact(&mut buffer)?;
        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn read_string_len32(&mut self) -> io::Result<String> {
        let len = self.read_u32()? as usize;
        let mut buffer = vec![0u8; len];
        self.read_exact(&mut buffer)?;
        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

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

#[cfg(test)]
mod bitmap_tests {
    use super::*;

    #[test]
    fn test_bitmap() {
        test_bitmap_for(1);
        test_bitmap_for(32);
        test_bitmap_for(96);
        test_bitmap_for(128);
        test_bitmap_for(129);
        test_bitmap_for(192);
        test_bitmap_for(256);
    }

    fn test_bitmap_for(len: usize) {
        let mut bitmap = Bitmap::new(len);
        assert_eq!(bitmap.len(), len);
        assert!(!bitmap.is_empty());
        assert!(!bitmap.filled());
        for i in 0..len {
            assert!(!bitmap.get(i));
            bitmap.set(i, true);
        }
        assert!(bitmap.filled());
        for i in 0..len {
            assert!(bitmap.get(i));
            bitmap.set(i, false);
        }
        assert!(!bitmap.filled());
    }
}