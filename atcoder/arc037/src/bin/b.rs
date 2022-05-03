use std::collections::VecDeque;

use proconio::input;

#[derive(Clone, Copy)]
pub struct Edge {
    start: usize,
    end: usize,
}
pub struct Solution {
    n: usize,
    m: usize,
    visited: Vec<bool>,
    graph: Vec<Edge>,
}

impl Solution {
    pub fn new(n: usize, m: usize, edges: Vec<(usize, usize)>) -> Self {
        let visited = vec![false; n];
        let mut graph: Vec<Edge> = Vec::new();
        for i in 0..m {
            let edge = Edge { start: edges[i].0 - 1, end: edges[i].1 - 1 };
            graph.push(edge);
        }
        Self {
            n,
            m,
            visited,
            graph
        }
    }

    pub fn run(&mut self) -> usize {
        let mut count_cycle = 0;
        for i in 0..self.n {
            if !self.visited[i] {
                if self.dfs(i) {
                    count_cycle += 1
                }
            }
        }
        return count_cycle
    }

    pub fn dfs(&mut self, i: usize) -> bool {
        let mut que: VecDeque<usize> = VecDeque::new();
        que.push_front(i);
        let mut has_no_cycle = true;
        while que.len() != 0 {
            let k = que.pop_back().unwrap();
            // 既に訪れていた場合は、閉路を持っていることになる
            if self.visited[k] {
                has_no_cycle = false;
            }
            // 訪れていない場合は閉路を持っていなく、辺を辿って探索する
            else {
                self.visited[k] = true;
                for i in 0..self.m {
                    let edge = self.graph[i];
                    if k == edge.start && !self.visited[edge.end] {
                        que.push_front(edge.end);
                    }
                    else if k == edge.end && !self.visited[edge.start] {
                        que.push_front(edge.start);
                    }
                }
            }
        }

        return has_no_cycle;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut solution = Solution::new(n, m, edges);
    let ans = solution.run();
    println!("{}", ans);
}
