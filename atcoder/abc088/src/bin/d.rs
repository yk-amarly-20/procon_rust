use std::vec;

use proconio::input;

const DX: [isize; 4] = [0, 1, 0, -1];
const DY: [isize; 4] = [1, 0, -1, 0];

pub struct Point {
    x: usize,
    y: usize,
}

pub struct Solution {
    height: usize,
    width: usize,
    maze: Vec<Vec<char>>,
    black_count: usize,
}

impl Solution {
    pub fn new(height: usize, width: usize, str_vec: Vec<String>) -> Self {
        let (maze, black_count) = get_char_vec_and_count_black(str_vec);
        Self {
            height,
            width,
            maze,
            black_count
        }
    }

    pub fn run(self) -> isize {
        let mut dist_map: Vec<Vec<isize>> = vec![vec![-1_isize; self.width]; self.height];
        dist_map[0][0] = 1;
        let current_position = Point { x: 0, y: 0 };
        self.dfs(&mut dist_map, current_position);

        let goal_dist = dist_map[self.height - 1][self.width - 1];
        if goal_dist < 0 {
            return goal_dist;
        } else {
            return self.height as isize * self.width as isize - goal_dist - self.black_count as isize;
        }
    }

    pub fn dfs(&self, dist_map: &mut Vec<Vec<isize>>, current: Point) {
        let current_dist = dist_map[current.y][current.x];
        for i in 0..DX.len() {
            let dx = DX[i];
            let dy = DY[i];
            let new_x = current.x as isize + dx;
            let new_y = current.y as isize + dy;
            if self.is_good_point(new_x, new_y) {
                let new_point = Point { x: new_x as usize, y: new_y as usize };
                // 未探索
                if dist_map[new_point.y][new_point.x] < 0 {
                    dist_map[new_point.y][new_point.x] = current_dist + 1;
                    self.dfs(dist_map, new_point);
                }
                // 探索済み
                else if dist_map[new_point.y][new_point.x] > current_dist + 1 {
                    dist_map[new_point.y][new_point.x] = current_dist + 1;
                    self.dfs(dist_map, new_point);
                }
            }
        }
    }

    pub fn is_good_point(&self, x: isize, y: isize) -> bool {
        let is_good_x = 0 <= x && x < self.width as isize;
        let is_good_y = 0 <= y && y < self.height as isize;
        if is_good_x && is_good_y {
            let is_white = self.maze[y as usize][x as usize] == '.';
            return is_white;
        }
        return false;
    }
}

pub fn get_char_vec_and_count_black(str_vec: Vec<String>) -> (Vec<Vec<char>>, usize) {
    let mut chars_vec: Vec<Vec<char>> = Vec::new();
    let mut black_counter = 0;
    for s in str_vec.into_iter() {
        let mut char_vec: Vec<char> = Vec::new();
        for c in s.chars().into_iter() {
            if c == '#' {
                black_counter += 1;
            }
            char_vec.push(c);
        }
        chars_vec.push(char_vec);
    }
    return (chars_vec, black_counter);
}

fn main() {
    input! {
        height: usize,
        width: usize,
        maze: [String; height],
    }

    let solution = Solution::new(height, width, maze);
    let ans = solution.run();
    println!("{}", ans);
}
