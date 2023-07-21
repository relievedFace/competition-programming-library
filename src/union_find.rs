#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            sizes: vec![1; n],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let x = self.parent[x];
            self.parent[x] = self.root(x);
            self.parent[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x != y {
            if self.sizes[x] < self.sizes[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent[y] = x;
            self.sizes[x] += self.sizes[y];
            true
        } else {
            false
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn size(&self, x: usize) -> usize {
        self.sizes[x]
    }

    pub fn root_size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.sizes[root]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uf() {
        //    0           3
        //  /   \       /   \
        // 1     2     4     6
        //           / | \   |
        //          5  7  8  9
        let mut uf = UnionFind::new(10);
        uf.unite(0, 1);
        uf.unite(0, 2);
        uf.unite(3, 4);
        uf.unite(3, 6);
        uf.unite(4, 5);
        uf.unite(4, 7);
        uf.unite(4, 8);
        uf.unite(6, 9);

        // test root
        assert_eq!(0, uf.root(0));
        assert_eq!(0, uf.root(1));
        assert_eq!(0, uf.root(2));
        assert_eq!(3, uf.root(3));
        assert_eq!(3, uf.root(4));
        assert_eq!(3, uf.root(5));
        assert_eq!(3, uf.root(6));
        assert_eq!(3, uf.root(7));
        assert_eq!(3, uf.root(8));
        assert_eq!(3, uf.root(9));

        // test same
        assert!(uf.same(0, 0));
        assert!(uf.same(0, 1));
        assert!(uf.same(0, 2));
        assert!(uf.same(1, 0));
        assert!(uf.same(1, 1));
        assert!(uf.same(1, 2));
        assert!(uf.same(2, 0));
        assert!(uf.same(2, 1));
        assert!(uf.same(2, 2));

        assert!(!uf.same(0, 3));
        assert!(!uf.same(1, 3));
        assert!(!uf.same(2, 8));

        assert!(uf.same(3, 8));
        assert!(uf.same(5, 9));
    }
}
