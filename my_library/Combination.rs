use itertools::Itertools;

// nCr
fn combinations(n: usize, r: usize) -> usize {
    (0..n).combinations(r).count()
}

// nPr
fn permutations(n: usize, r: usize) -> usize {
    (0..n).permutations(r).count()
}

// nHr
fn combinations_with_replacement(n: usize, r: usize) -> usize {
    (0..n).combinations_with_replacement(r).count()
}
