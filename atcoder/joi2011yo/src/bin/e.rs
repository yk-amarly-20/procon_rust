use proconio::input;

const DX: [isize; 4] = [0, 1, 0, -1];
const DY: [isize; 4] = [1, 0, -1, 0];

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    pub fn get_default_point() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }
}

pub struct Solution {
    height: usize,
    width: usize,
    n: usize,
    // スタート地点、数字マスの座標
    special_points: Vec<Point>,
    map: Vec<Vec<char>>,
}

impl Solution {
    pub fn new(height: usize, width: usize, n: usize, str_map: Vec<String>) -> Self {
        let mut map: Vec<Vec<char>> = Vec::new();
        for s in str_map.into_iter() {
            let mut chars: Vec<char> = Vec::new();
            for c in s.chars().into_iter() {
                chars.push(c);
            }
            map.push(chars);
        }
        let mut special_points: Vec<Point> = vec![Point::get_default_point(); n + 1];


        for i in 0..height {
            for j in 0..width {
                if map[i][j] == 'S' {
                    // スタート地点
                    special_points[0] = Point::new(j, i);
                }
                else if map[i][j] != '.' && map[i][j] != 'X' {
                    // 数値
                    let num = map[i][j].to_string().parse::<usize>().unwrap() - 1;
                    special_points[num] = Point::new(j, i);
                }
            }
        }

        Self {
            height: height,
            width: width,
            n: n,
            special_points: special_points,
            map: map
        }
    }

    pub fn run(&self) -> usize {
        let mut total_time = 0;
        for i in 0..self.n {
            total_time += self.batch(i);
        }

        return total_time;
    }

    pub fn batch(&self, i: usize) -> usize {
        let start_point = self.special_points[i];
        let goal_point = self.special_points[i + 1];
        let mut temp_map = vec![vec![-1_isize; self.width]; self.height];
        temp_map[start_point.y][start_point.x] = 0;
        self.dfs(&mut temp_map, start_point);
        return temp_map[goal_point.y][goal_point.x] as usize;
    }

    pub fn dfs(&self, map: &mut Vec<Vec<isize>>, present: Point) {
        let present_dist = map[present.y][present.x];
        for i in 0..4 {
            let new_x = present.x as isize + DX[i];
            let new_y = present.y as isize + DY[i];
            if 0 <= new_x && new_x < self.width as isize && 0 <= new_y && new_y < self.height as isize {
                let new_point = Point::new(new_x as usize, new_y as usize);
                if self.map[new_point.y][new_point.x] != 'X' {
                    // 未探索の場合
                    if map[new_point.y][new_point.x] < 0 {
                        map[new_point.y][new_point.x] = present_dist + 1;
                        self.dfs(map, new_point);
                    }
                    // 探索済みの場合
                    else if map[new_point.y][new_point.x] > present_dist + 1 {
                        map[new_point.y][new_point.x] = present_dist + 1;
                        self.dfs(map, new_point);
                    }
                }
            }
        }
    }
}

fn main() {
    input! {
        height: usize,
        width: usize,
        n: usize,
        str_map: [String; height],
    }

    let solution = Solution::new(height, width, n, str_map);
    let ans = solution.run();
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {}
}
