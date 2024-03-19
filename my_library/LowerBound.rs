/**
 * 二分探索
 * containerは昇順にソートされている必要がある
 * containerの中でvalue以上の値が初めて現れる位置を返す
 * なければcontainer.len()を返す
 * 常にcontainer[left] < value <= container[right]となるようにする
 */
fn lower_bound(container: Vec<i64>, value: i64) -> usize {
    let mut left: usize = 0;
    let mut right: usize = container.len() as usize;
    if value <= container[0] {
        return 0;
    }
    while right - left > 1 {
        let mid = (left + right) / 2;
        if container[mid] < value {
            left = mid;
        } else {
            right = mid;
        }
    }
    return right;
}

/**
 * 二分探索
 * containerは昇順にソートされている必要がある
 * containerの中でvalueより大きい値が初めて現れる位置を返す
 * なければcontainer.len()を返す
 * 常にcontainer[left] <= value < container[right]となるようにする
 */
fn upper_bound(container: Vec<i64>, value: i64) -> usize {
    let mut left: usize = 0;
    let mut right: usize = container.len() as usize;
    if value < container[0] {
        return 0;
    }
    while right - left > 1 {
        let mid = (left + right) / 2;
        if container[mid] <= value {
            left = mid;
        } else {
            right = mid;
        }
    }
    return right;
}

/**
 * 二分探索
 * Fn(i64) -> boolの関数fが与えられたとき、left<=x<rightを満たすxの内f(x)がtrueとなる最大のxを返す
 * f(x)がtrueとなるxが存在しない場合はleftを返す
 */
fn binary_max_search(left: i64, right: i64, f: &dyn Fn(i64) -> bool) -> i64 {
    let mut left = left;
    let mut right = right;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if f(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    return left;
}

/**
 * 二分探索
 * Fn(i64) -> boolの関数fが与えられたとき、left< x<=rightを満たすxの内f(x)がtrueとなる最小のxを返す
 * f(x)がtrueとなるxが存在しない場合はrightを返す
 */
fn binary_min_search(left: i64, right, f: &dyn Fn(i64) -> bool) -> i64 {
    let mut left = left;
    let mut right = right;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if f(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    return right;
}
