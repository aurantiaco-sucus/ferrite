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