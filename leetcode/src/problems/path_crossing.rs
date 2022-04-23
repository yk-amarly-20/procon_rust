pub struct Point {
  x: i32,
  y: i32,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x: x, y: y }
  }
}

struct Solution {}

impl Solution {
  pub fn is_path_crossing(path: String) -> bool {
    let mut history: Vec<(i32, i32)> = Vec::new();
    history.push((0, 0));
    let mut present_position = Point::new(0, 0);
    for c in path.chars().into_iter() {
      match c {
          'N' => {
            present_position.y += 1;
            if check_point(&history, Point{x: present_position.x, y: present_position.y}) {
              return true;
            }
            history.push((present_position.x, present_position.y));
          }

          'S' => {
            present_position.y -= 1;
            if check_point(&history, Point{x: present_position.x, y: present_position.y}) {
              return true;
            }
            history.push((present_position.x, present_position.y));
          }

          'E' => {
            present_position.x += 1;
            if check_point(&history, Point{x: present_position.x, y: present_position.y}) {
              return true;
            }
            history.push((present_position.x, present_position.y));
          }

          'W' => {
            present_position.x -= 1;
            if check_point(&history, Point{x: present_position.x, y: present_position.y}) {
              return true;
            }
            history.push((present_position.x, present_position.y));
          }
          _ => continue
      }
    }
    return false;
  }
}

pub fn check_point(history: &Vec<(i32, i32)>, position: Point) -> bool {
  for (i, j) in history.into_iter() {
    if *i == position.x && *j == position.y {
      return true;
    }
  }

  return false;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::is_path_crossing("NESWW".to_string()), true);
  }
}
