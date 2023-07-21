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

    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.sizes[root]
    }
}
