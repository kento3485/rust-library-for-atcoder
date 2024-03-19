struct UnionFind {
    par: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self { par: vec![-1; n] }
    }

    fn leader(&mut self, n: usize) -> usize {
        if self.par[n] < 0 {
            return n;
        }
        let ld = self.leader(self.par[n] as usize);
        self.par[n] = ld as i64;
        return ld;
    }

    fn size(&mut self, n: usize) -> usize {
        if self.par[n] < 0 {
            return (self.par[n] * -1) as usize;
        }
        let ld = self.leader(n);
        return (self.par[ld as usize] * -1) as usize;
    }

    fn merge(&mut self, n: usize, m: usize) -> bool {
        let leader_n = self.leader(n);
        let leader_m = self.leader(m);
        if leader_n == leader_m {
            return false;
        }
        self.par[leader_n] += self.par[leader_m];
        self.par[leader_m] = leader_n as i64;
        return true;
    }

    fn same(&mut self, n: usize, m: usize) -> bool {
        if self.leader(n) == self.leader(m) {
            return true;
        }
        return false;
    }

    fn groups(&self) -> usize {
        let mut cnt: usize = 0;
        for ld in &self.par {
            if ld < &0 {
                cnt += 1;
            }
        }
        return cnt;
    }
}
