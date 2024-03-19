fn num_to_char(x: i64) -> char {
    if x == 0 {
        return 'M';
    } else {
        return 'P';
    }
}

fn f_plus(x: i64) -> i64 {
    return x;
}

fn f_minus(x: i64) -> i64 {
    return -x;
}

fn sum(x: i64, y: i64) -> i64 {
    return x + y;
}

fn inv_sum(x: i64, y: i64) -> i64 {
    return x - y;
}

/**
 * seqをn桁の二進数とみなして、i桁目の値をVec[i]にして返す。
 * すなわち、Vec[i] = seq >> i & 1
 */
fn seq_to_vec(seq: i64, n: usize) -> Vec<i64> {
    let mut res = Vec::new();
    for i in 0..n {
        res.push(seq >> i & 1);
    }
    return res;
}

/**
 * 半分全列挙を行う。
 * lの各要素に対して、f_plusとf_minusのいずれかを適用して、sum関数によって足し合わせた結果がtargetとなるような適用の仕方があれば、
 * それをseqとして返す。seqをn桁の二進数とみなして、i桁目が1ならばf_plusを、0ならばf_minusを適用したとみなす。
 * 計算量: O(n*2^(n/2)) (ただし、n=l.len())
 * 全列挙する場合の計算量(参考): O(2^n)
 */
fn half_complete_enumeration(
    l: &[i64],
    f_plus: &dyn Fn(i64) -> i64,
    f_minus: &dyn Fn(i64) -> i64,
    sum: &dyn Fn(i64, i64) -> i64,
    inv_sum: &dyn Fn(i64, i64) -> i64,
    initial_sum: i64,
    target: i64,
) -> Option<i64> {
    let mid = l.len() / 2;
    let (first_half_hs, first_half_hm) =
        half_complete_enumeration_set_and_map(&l[..mid], f_plus, f_minus, sum, initial_sum);
    let (second_half_hs, second_half_hm) =
        half_complete_enumeration_set_and_map(&l[mid..], f_plus, f_minus, sum, initial_sum);

    for s in first_half_hs {
        let t = inv_sum(target, s);
        if second_half_hs.contains(&t) {
            return Some((second_half_hm[&t] << mid) | first_half_hm[&s]);
        }
    }
    return None;
}

fn half_complete_enumeration_set(
    l_half: &[i64],
    f_plus: &dyn Fn(i64) -> i64,
    f_minus: &dyn Fn(i64) -> i64,
    sum: &dyn Fn(i64, i64) -> i64,
    initial_sum: i64,
) -> HashSet<i64> {
    let (hs, _) = half_complete_enumeration_set_and_map(l_half, f_plus, f_minus, sum, initial_sum);
    return hs;
}

fn half_complete_enumeration_map(
    l_half: &[i64],
    f_plus: &dyn Fn(i64) -> i64,
    f_minus: &dyn Fn(i64) -> i64,
    sum: &dyn Fn(i64, i64) -> i64,
    initial_sum: i64,
) -> HashMap<i64, i64> {
    let (_, hm) = half_complete_enumeration_set_and_map(l_half, f_plus, f_minus, sum, initial_sum);
    return hm;
}

/**
 * l_halfの各要素に対して、f_plusとf_minusのいずれかを適用して、sum関数によって足し合わせた結果をsとしたときに以下を返す。
 * sをHashSetに格納して返す。
 * sをkey、i番目の要素にf_plusを適用したなら二進数とみなしたときのi桁目が1、f_minusならば0となるseqをvalueとして、HashMapに格納して返す。
 * HashSetとHashMapに格納して返す。
 */
fn half_complete_enumeration_set_and_map(
    l_half: &[i64],
    f_plus: &dyn Fn(i64) -> i64,
    f_minus: &dyn Fn(i64) -> i64,
    sum: &dyn Fn(i64, i64) -> i64,
    initial_sum: i64,
) -> (HashSet<i64>, HashMap<i64, i64>) {
    let mut hs: HashSet<i64> = HashSet::new();
    let mut hm: HashMap<i64, i64> = HashMap::new();
    for x in 0..(1 << l_half.len()) {
        let mut s = initial_sum;
        for i in 0..l_half.len() {
            s = sum(
                s,
                if x >> i & 1 == 1 {
                    f_plus(l_half[i])
                } else {
                    f_minus(l_half[i])
                },
            );
        }
        hs.insert(s);
        hm.insert(s, x);
    }
    return (hs, hm);
}
