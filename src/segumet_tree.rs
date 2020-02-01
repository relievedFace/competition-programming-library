pub struct SegmentTree<T, F> {
    n: usize,
    data: Vec<T>,
    id: T,
    op: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    pub fn new(n: usize, id: T, op: F) -> SegmentTree<T, F> {
        let mut len = 1;
        while len < n {
            len *= 2;
        }
        SegmentTree {
            n: n,
            data: vec![id; 2 * len - 1],
            id: id,
            op: op,
        }
    }
    pub fn update(&mut self, mut i: usize, x: T) {
        i += self.n - 1;
        self.data[i] = x;
        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = (self.op)(self.data[i * 2 + 1], self.data[i * 2 + 2]);
        }
    }

    pub fn query(&self, a: usize, b: usize) -> T {
        self.inner_query(a, b, 0, 0, self.n)
    }

    fn inner_query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            self.id
        } else if a <= l && r <= b {
            self.data[k]
        } else {
            let vl = self.inner_query(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.inner_query(a, b, k * 2 + 2, (l + r) / 2, r);
            (self.op)(vl, vr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test::Bencher;
    use super::*;
    use rand::prelude::*;
    const SIZE: usize = 1 << 8;
    const BENCH_SIZE: usize = 1 << 16;

    #[test]
    fn max() {
        let mut seg = SegmentTree::new(SIZE, std::i32::MIN, std::cmp::max);
        let random_array: Vec<i32> = (0..SIZE).map(|_| rand::random()).collect();
        for (i, &x) in random_array.iter().enumerate() {
            seg.update(i as usize, x);
        }

        for i in 0..SIZE {
            for j in 0..i {
                assert_eq!(seg.query(j, i), *random_array[j..i].iter().max().unwrap());
            }
        }
    }
}
