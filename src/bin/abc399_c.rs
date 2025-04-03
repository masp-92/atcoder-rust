use proconio::input;
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    // 初期化
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    // 根を探す（経路圧縮あり）
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // 経路圧縮
        }
        self.parent[x]
    }

    // 結合（union by size）
    fn union(&mut self, x: usize, y: usize) -> bool {
        let mut x_root = self.find(x);
        let mut y_root = self.find(y);
        if x_root == y_root {
            return false;
        }
        if self.size[x_root] < self.size[y_root] {
            std::mem::swap(&mut x_root, &mut y_root);
        }
        self.parent[y_root] = x_root;
        self.size[x_root] += self.size[y_root];
        true
    }

    // 同じグループかどうか
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    // グループのサイズを取得
    fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut ans = 0;
    let mut uf = UnionFind::new(n);
    for (mut u, mut v) in edges {
        u -= 1;
        v -= 1;
        if uf.same(u, v) {
            ans += 1;
        }

        uf.union(u, v);
    }
    println!("{}", ans);
}
