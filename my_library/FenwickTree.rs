struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> FenwickTree {
        FenwickTree {
            n: n,
            data: vec![0; n],
        }
    }

    fn from_slice(v: &[i64]) -> FenwickTree {
        let mut ft = FenwickTree::new(v.len());
        for i in 0..v.len() {
            ft.add(i, v[i]);
        }
        ft
    }

    /**
     * i番目の要素にxを加算
     */
    fn add(&mut self, i: usize, x: i64) {
        let mut idx = i;
        while idx < self.n {
            self.data[idx] += x;
            idx |= idx + 1;
        }
    }

    /**
     * [0, i)の和
     */
    fn sum(&self, i: usize) -> i64 {
        let mut idx = i;
        let mut res = 0;
        while idx > 0 {
            res += self.data[idx - 1];
            idx = idx & idx - 1;
        }
        res
    }

    /**
     * [i, j)の和
     */
    fn sum_range(&self, i: usize, j: usize) -> i64 {
        self.sum(j) - self.sum(i)
    }

    /**
     * i番目の要素を取得
     */
    fn get(&self, i: usize) -> i64 {
        self.sum_range(i, i + 1)
    }
}
