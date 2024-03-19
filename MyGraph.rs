use std::collections::VecDeque;

struct SimpleGraph {
    n: usize,
    edges: Vec<Vec<usize>>,
}

impl SimpleGraph {
    fn new(n: usize, edges: Vec<Vec<usize>>) -> Self {
        MyGraph { n: n, edges: edges }
    }
}
