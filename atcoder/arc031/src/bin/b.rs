// TODO: 何故かWA、原因がわからん
use proconio::input;

const DX: [isize; 4] = [1, 0, -1, 0];
const DY: [isize; 4] = [0, 1, 0, -1];
const HEIGHT: isize = 10;
const WIDTH: isize = 10;

pub struct Solution {
    land: Vec<Vec<char>>,
}

impl Solution {
    pub fn new(land: Vec<String>) -> Self {
        let mut vec_chars: Vec<Vec<char>> = Vec::new();

        for i in 0..land.len() {
            let mut chars: Vec<char> = Vec::new();
            for c in land[i].chars().into_iter() {
                chars.push(c)
            }
            vec_chars.push(chars);
        }
        Self {
            land: vec_chars
        }
    }

    pub fn run(&self) -> bool {
        let height = 10;
        let width = 10;
        for i in 0..height {
            for j in 0..width {
                let mut new_land = self.land.clone();
                if self.land[j][i] == 'x' {
                    self.dfs(j, i, &mut new_land);
                }
                if self.is_all_ocean(&new_land) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn dfs(&self, x: usize, y: usize, new_land: &mut Vec<Vec<char>>) {
        // oを✖︎に書き換える
        new_land[y][x] = 'x';
        for i in 0..DX.len() {
            let new_x = x as isize + DX[i];
            let new_y = y as isize + DY[i];
            if new_x >= 0 && new_x < WIDTH && new_y >= 0 && new_y < HEIGHT {
                if new_land[new_y as usize][new_x as usize] == 'o' {
                    self.dfs(new_x as usize, new_y as usize, new_land);
                }
            }
        }
    }

    pub fn is_all_ocean(&self, land: &Vec<Vec<char>>) -> bool {
        for i in 0..(HEIGHT as usize) {
            for j in 0..(WIDTH as usize) {
                if land[j][i] == 'o' {
                    return false;
                }
            }
        }
        return true;
    }
}

fn main() {
    input! {
        land: [String; 10],
    }

    let solution = Solution::new(land);
    if solution.run() {
        println!("YES");
    } else {
        println!("NO");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        // 一つだけ陸地のパターン1
        let mut case_s_1 = vec![
            "xxxxxxxxxo".to_string()
        ];
        for _ in 0..9 {
            case_s_1.push("xxxxxxxxxx".to_string());
        }
        let mut solution = Solution::new(case_s_1);
        assert_eq!(solution.run(), true);

        // 一つだけの陸地のパターン2
        let mut case_s_2: Vec<String> = Vec::new();
        for _ in 0..3 {
            case_s_2.push("xxxxxxxxxx".to_string());
        }
        case_s_2.push("xxxxoxxxxx".to_string());
        for _ in 0..6 {
            case_s_2.push("xxxxxxxxxx".to_string());
        }
        solution = Solution::new(case_s_2);
        assert_eq!(solution.run(), true);

        // 一つだけ海のパターン
        let mut case_s_3: Vec<String> = Vec::new();
        case_s_3.push("ooooooooox".to_string());
        for _ in 0..9 {
            case_s_3.push("oooooooooo".to_string());
        }
        solution = Solution::new(case_s_3);
        assert_eq!(solution.run(), true);

        // 一つだけ海のパターン2
        let mut case_s_4: Vec<String> = Vec::new();
        for _ in 0..3 {
            case_s_4.push("oooooooooo".to_string());
        }
        case_s_4.push("ooooxooooo".to_string());
        for _ in 0..6 {
            case_s_4.push("oooooooooo".to_string());
        }
        solution = Solution::new(case_s_4);
        assert_eq!(solution.run(), true);

        // NGパターン1
        let mut case_s_5: Vec<String> = Vec::new();
        case_s_5.push("xxxxxxxxxo".to_string());
        case_s_5.push("xxxxxxxxxx".to_string());
        for _ in 0..8 {
            case_s_5.push("ooooooooxx".to_string());
        }
        solution = Solution::new(case_s_5);
        assert_eq!(solution.run(), false);

        // OKパターン
        let mut case_s_6: Vec<String> = Vec::new();
        case_s_6.push("oxoooooooo".to_string());
        for _ in 0..2 {
            case_s_6.push("oooooooooo".to_string());
        }
        case_s_6.push("ooooxooooo".to_string());
        for _ in 0..6 {
            case_s_6.push("oooooooooo".to_string());
        }
        solution = Solution::new(case_s_6);
        assert_eq!(solution.run(), true);
    }
}
