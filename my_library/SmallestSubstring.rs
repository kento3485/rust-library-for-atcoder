/**
 * next_idx[i][j] = s[i]以降で最初に文字j(アルファベット小文字0~25)が出現する位置のインデックスと
 * なるようなnext_idxを計算して返す。
 */
fn calculate_next_idx(s: &str, n: usize) -> Vec<Vec<usize>> {
    let mut next_idx = Vec::new();
    let mut idxs = vec![n; 26];
    let s = s.chars().collect::<Vec<char>>();
    for i in (0..n).rev() {
        idxs[(s[i] as u32 - 97) as usize] = i;
        next_idx.push(idxs.clone());
    }
    next_idx.reverse();
    return next_idx;
}
