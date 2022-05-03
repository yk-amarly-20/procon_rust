use std::vec;
use proconio::input;

const DX: [isize; 4] = [1, 0, -1, 0];
const DY: [isize; 4] = [0, 1, 0, -1];

#[derive(Debug, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

pub struct Solution {
    r: usize,
    c: usize,
    start: Point,
    goal: Point,
    map: Vec<Vec<char>>,
    // 最短距離のベクタ
    // 探索済みなら0以上の数値, 未探索なら-1
    dist: Vec<Vec<isize>>,
}

impl Solution {
    pub fn new(r: usize, c: usize, sx: usize, sy: usize, gx: usize, gy: usize, str_map: Vec<String>) -> Self {
        let dist: Vec<Vec<isize>> = vec![vec![-1_isize; c]; r];
        let start = Point { x: sx - 1, y: sy - 1 };
        let goal = Point { x: gx - 1, y: gy - 1 };
        let mut map: Vec<Vec<char>> = Vec::new();
        for s in str_map.into_iter() {
            let mut chars: Vec<char> = Vec::new();
            for c in s.chars().into_iter() {
                chars.push(c);
            }
            map.push(chars);
        }

        return Self {
            r: r,
            c: c,
            start: start,
            goal: goal,
            map: map,
            dist: dist,
        };
    }

    pub fn run(&mut self) -> isize {
        self.dist[self.start.y][self.start.x] = 0;
        self.dfs(&self.start.clone());
        return self.dist[self.goal.y][self.goal.x];
    }

    pub fn dfs(&mut self, present: &Point) {
        // 自分の四方を幅優先探索
        let present_dist = self.dist[present.y][present.x];
        for i in 0..4 {
            let new_x = present.x as isize + DX[i];
            let new_y = present.y as isize + DY[i];
            if 0 <= new_x && new_x  < self.c as isize && 0 <= new_y && new_y < self.r as isize {
                let new_point = Point { x: new_x as usize, y: new_y as usize };
                if self.map[new_point.y][new_point.x] == '.' {
                    // 未探索の場合
                    if self.dist[new_point.y][new_point.x] < 0 {
                        self.dist[new_point.y][new_point.x] = present_dist + 1;
                        self.dfs(&new_point);
                    }
                    // 探索済みの場合、値の更新が走るときはもう一度探索
                    else {
                        if self.dist[new_point.y][new_point.x] > present_dist + 1 {
                            self.dist[new_point.y][new_point.x] = present_dist + 1;
                            self.dfs(&new_point);
                        }
                    }
                }
            }
        }

    }
}

fn main() {
    input! {
        r: usize,
        c: usize,
        sy: usize,
        sx: usize,
        gy: usize,
        gx: usize,
        str_map: [String; r],
    }

    let mut solution = Solution::new(r, c, sx, sy, gx, gy, str_map);
    let ans = solution.run();
    println!("{}", ans);
}
