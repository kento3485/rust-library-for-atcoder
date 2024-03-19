fn dijkstra(s: usize, d: Vec<Vec<i64>>) -> Vec<i64> {
    /**
     * s : 開始地点
     * d : 隣接行列 d[i][j]はiからjへ向かう辺の重み(辺がない場合はstd::i64::MAXなどにする。)
     * return : 各頂点iへの最短距離を返す。
     *
     * 仮定
     * - グラフは連結であり、sから全ての頂点へのパスが存在する。
     *
     * 計算量 : O(n^2)
     */
    let INF = std::i64::MAX;
    // 暫定の最短距離を格納する配列
    let mut dist: Vec<i64> = vec![INF; d.len()];
    dist[s] = 0;
    // 最短距離が確定した頂点を格納する配列
    let mut fixed: HashMap<usize, i64> = HashMap::<usize, i64>::new();
    // 周辺部を格納するための優先度付きキュー
    let mut q = std::collections::BinaryHeap::new();
    q.push((0, s));
    while let Some((inv_w, v)) = q.pop() {
        if fixed.contains_key(&v) {
            continue;
        }
        let w = -1 * inv_w;
        fixed.insert(v, w);
        for i in 0..d[v].len() {
            if dist[i]
                > if d[v][i] == INF || w == INF {
                    INF
                } else {
                    w + d[v][i]
                }
            {
                dist[i] = w + d[v][i];
                q.push((-1 * dist[i], i));
            }
        }
    }
    let mut res = vec![];
    for i in 0..d.len() {
        res.push(fixed[&i]);
    }
    return res;
}
