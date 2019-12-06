pub trait BinarySearch<T: Ord> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = low + (high - low) / 2;
            match self[mid].cmp(x) {
                std::cmp::Ordering::Less => low = mid + 1,
                _ => high = mid,
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = low + (high - low) / 2;
            match self[mid].cmp(x) {
                std::cmp::Ordering::Greater => high = mid,
                _ => low = mid + 1,
            }
        }
        high
    }
}
