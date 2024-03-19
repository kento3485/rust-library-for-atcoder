/**
 * nの逆元を返す。
 * 計算量: O(log(modulo))
 */
fn mod_inv(n: usize, modulo: usize) -> usize {
    mod_pow(n, modulo - 2, modulo)
}

/**
 * n^mをmoduloで割った余りを返す。
 * 計算量: O(log(m))
 */
fn mod_pow(n: usize, m: usize, modulo: usize) -> usize {
    let mut n = n % modulo;
    let mut m = m;
    let mut res = 1;
    while m > 0 {
        if m & 1 == 1 {
            res = (res * n) % modulo;
        }
        n = (n * n) % modulo;
        m >>= 1;
    }
    return res;
}
