
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