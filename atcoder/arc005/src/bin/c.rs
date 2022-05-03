// TODO: よくわからんな...
use std::collections::VecDeque;
use proconio::input;

const DX: [isize; 4] = [0, 1, 0, -1];
const DY: [isize; 4] = [1, 0, -1, 0];

pub struct PointCost {
    point: Point,
    cost: isize,
}

pub struct Point {
    x: usize,
    y: usize,
}

pub struct Solution {
    height: usize,
    width: usize,
    map: Vec<Vec<char>>,
    // コストマップ
    // 未探索なら-1、探索済みならそこまでたどり着く最低コスト(最大2)
    costs: Vec<Vec<isize>>,
    // 座標、そこまでたどり着くコストを入れておくqueue
    post_costs: VecDeque<PointCost>,
}

impl Solution {
    pub fn new(height: usize, width: usize, str_map: Vec<String>) -> Self {
        let (map, start) = get_char_map_and_get_start_and_goal(str_map);
        let costs = vec![vec![-1; width]; height];
        let mut post_costs: VecDeque<PointCost> = VecDeque::new();
        post_costs.push_back(PointCost { point: start, cost: 0 });
        return Self {
            height,
            width,
            map,
            costs,
            post_costs
        }
    }

    pub fn run(&mut self) -> bool {
        while !self.post_costs.is_empty() {
            let post_cost = self.post_costs.pop_front().unwrap();
            let point = post_cost.point;
            let cost = post_cost.cost;
            for i in 0..DX.len() {
                let new_x = point.x as isize + DX[i];
                let new_y = point.y as isize + DY[i];
                if self.is_good_point(new_x, new_y) {
                    let new_point = Point { x: new_x as usize, y: new_y as usize };

                    if self.costs[new_point.y][new_point.x] >= 0 && self.costs[new_point.y][new_point.x] <= cost {
                        // 探索済みでかつ再探索する必要なし
                        continue;
                    }

                    // 新しい点が壁だった場合
                    if self.map[new_point.y][new_point.x] == '#' {
                        // 動く前の点のコストが2以上であった場合、壁を壊すことができない
                        // そうでない場合、新しい点のコストをcost + 1で更新
                        if cost >= 2 {
                            continue;
                        }
                        self.costs[new_point.y][new_point.x] = cost + 1;
                        // self.post_costs.push_back((new_point, cost + 1));
                        self.post_costs.push_back(PointCost { point: new_point, cost: cost + 1 });
                    }
                    else if self.map[new_point.y][new_point.x] == 'g' {
                        // ゴールできた場合終わり
                        return true;
                    }
                    else {
                        self.costs[new_point.y][new_point.x] = cost;
                        // self.post_costs.push_back((new_point, cost));
                        self.post_costs.push_back(PointCost { point: new_point, cost: cost });
                    }
                }
            }
        }
        return false;
    }

    pub fn is_good_point(&self, x: isize, y: isize) -> bool {
        let is_good_x = 0 <= x && x < self.width as isize;
        let is_good_y = 0 <= y && y < self.height as isize;
        return is_good_x && is_good_y;
    }
}

pub fn get_char_map_and_get_start_and_goal(str_map: Vec<String>) -> (Vec<Vec<char>>, Point) {
    let mut chars_vec: Vec<Vec<char>> = Vec::new();
    let mut start = Point { x: 0, y: 0 };
    for i in 0..str_map.len() {
        let mut char_vec: Vec<char> = Vec::new();
        for c in str_map[i].chars().into_iter() {
            char_vec.push(c);
        }
        for j in 0..char_vec.len() {
            if char_vec[j] == 's' {
                // スタート地点
                let start = Point { x: j, y: i };
            }
        }
        chars_vec.push(char_vec);
    }
    return (chars_vec, start);
}

fn main() {
    input! {
        height: usize,
        width: usize,
        str_map: [String; height],
    }

    let mut solution = Solution::new(height, width, str_map);
    let ans = solution.run();
    if ans {
        println!("YES");
    } else {
        println!("NO");
    }
}
