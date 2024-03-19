use proconio::{fastout, input};
use std::collections::VecDeque;

/**
 * 木を表す構造体。
 * n: 頂点数
 * edges: 隣接リスト
 * aとbを結ぶ辺が存在するとき、edges[a]にbが、edges[b]にaが含まれる。
 */
struct Tree {
    n: usize,
    edges: Vec<Vec<usize>>,
}

impl Tree {
    fn new(n: usize, edges: Vec<Vec<usize>>) -> Self {
        Tree { n: n, edges: edges }
    }
    /**
     * startからterminalまでの距離を返す。
     */
    fn dist(&self, start: usize, terminal: usize) -> usize {
        let mut used: Vec<bool> = vec![false; self.n];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::from([]);
        queue.push_back((start, 0));
        let mut res: usize = 0;
        while let Some((cur, d)) = queue.pop_front() {
            if used[cur] {
                continue;
            }
            if cur == terminal {
                res = d;
                break;
            }
            used[cur] = true;
            for next in &self.edges[cur] {
                if !used[*next] {
                    queue.push_back((*next, d + 1));
                }
            }
        }
        return res;
    }
    /**
     * startから各頂点への距離を返す。
     */
    fn dist_from(&self, start: usize) -> Vec<usize> {
        let mut dist: Vec<i64> = vec![-1; self.n];
        let mut queue: VecDeque<(usize, i64)> = VecDeque::from([]);
        queue.push_back((start, 0));
        while let Some((cur, d)) = queue.pop_front() {
            if dist[cur] >= 0 {
                continue;
            }
            dist[cur] = d;
            for next in &self.edges[cur] {
                if dist[*next] < 0 {
                    queue.push_back((*next, d + 1));
                }
            }
        }

        let dist: Vec<usize> = dist.into_iter().map(|x| x as usize).collect();
        return dist;
    }
    /**
     * startから最も遠い頂点のリストを返す。
     */
    fn most_distant_nodes(&self, u: usize) -> Vec<usize> {
        let dist_from_u = self.dist_from(u);

        let mut argmax: Vec<usize> = Vec::from([0]);
        let mut max: usize = dist_from_u[0];

        for i in 1..self.n {
            if max < dist_from_u[i] {
                max = dist_from_u[i];
                argmax = Vec::from([i]);
            } else if max == dist_from_u[i] {
                argmax.push(i);
            }
        }
        return argmax;
    }
    /**
     * 木の直径を返す。
     */
    fn diameter(&self) -> usize {
        let node1 = self.most_distant_nodes(0)[0];
        let node2 = self.most_distant_nodes(node1)[0];

        return self.dist(node1, node2);
    }
}

/**
 * 1-indexedの辺のリストから木を作成する。
 */
fn create_tree_from_1idxed_ablist(n: usize, ablist: &[(usize, usize)]) -> Tree {
    let mut edges: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n - 1 {
        let (a, b) = ablist[i];
        edges[a - 1].push(b - 1);
        edges[b - 1].push(a - 1);
    }
    Tree { n, edges }
}

/**
 * 標準入力から木を作成する。
 * 1行目に頂点数n、続くn-1行に各辺の両端の頂点番号a,bが空白区切りで与えられる。
 * 各頂点番号は1からnまでの整数である。
 */
fn create_tree_from_stdin() -> Tree {
    input! {
        n:usize,
        ablist:[(usize,usize);(n-1)]
    }
    create_tree_from_1idxed_ablist(n, ablist.as_slice())
}
