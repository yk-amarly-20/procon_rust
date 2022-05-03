use std::collections::VecDeque;

use proconio::input;

const DX: [isize; 4] = [0, 1, -1, 0];
const DY: [isize; 4] = [1, 0, 0, -1];

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}
pub struct Solution {
    height: usize,
    width: usize,
    map: Vec<Vec<char>>,
    black_queue: VecDeque<Point>,
}

impl Solution {
    pub fn new(height: usize, width: usize, str_map: Vec<String>) -> Self {
        let (map, black_queue) = get_char_vec_and_black_queue(str_map);
        Self {
            height,
            width,
            map,
            black_queue,
        }
    }

    pub fn run(&mut self) -> usize {
        let mut count = 0;
        while !self.black_queue.is_empty() {
            // 操作一回分
            for _ in 0..self.black_queue.len() {
                let black_point = self.black_queue.pop_front().unwrap();
                // 4方向の探索
                // 範囲内かつ白マスなら、マスを黒にしてqueueに追加
                self.dfs(black_point);
            }
            count += 1;
        }
        return count - 1;
    }

    // マス一つ分の操作
    pub fn dfs(&mut self, black_point: Point) {
        for i in 0..DX.len() {
            let new_x = black_point.x as isize + DX[i];
            let new_y = black_point.y as isize + DY[i];
            if self.is_good_point(new_x, new_y) {
                // 黒ます変更、かつqueueに追加
                let new_point = Point { x: new_x as usize, y: new_y as usize };
                self.map[new_point.y][new_point.x] = '#';
                self.black_queue.push_back(new_point);
            }
        }
    }

    pub fn is_good_point(&self, x: isize, y: isize) -> bool {
        // 正常位置かつマス目が白いか探索
        let mut is_white = false;
        let is_good_x = 0 <= x && x < self.width as isize;
        let is_good_y = 0 <= y && y < self.height as isize;
        if is_good_x && is_good_y {
            is_white = self.map[y as usize][x as usize] == '.';
        }
        return is_white;
    }
}

pub fn get_char_vec_and_black_queue(str_map: Vec<String>) -> (Vec<Vec<char>>, VecDeque<Point>) {
    let mut chars_vec: Vec<Vec<char>> = Vec::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    for i in 0..str_map.len() {
        let mut char_vec: Vec<char> = Vec::new();
        for c in str_map[i].chars().into_iter() {
            char_vec.push(c);
        }
        for j in 0..char_vec.len() {
            if char_vec[j] == '#' {
                queue.push_back(Point { x: j, y: i });
            }
        }
        chars_vec.push(char_vec);
    }
    return (chars_vec, queue);
}

fn main() {
    input! {
        height: usize,
        width: usize,
        str_map: [String; height],
    }

    let mut solution = Solution::new(height, width, str_map);
    let ans = solution.run();
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {}
}
