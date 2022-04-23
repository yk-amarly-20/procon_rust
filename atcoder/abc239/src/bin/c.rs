use std::vec;

use proconio::input;

pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x: x, y: y }
    }

    pub fn get_all_knights(&self) -> Vec<Point> {
        let mut knights: Vec<Point> = Vec::new();
        let x_diff: Vec<isize> = vec![1, 1, -1, -1, 2, 2, -2, -2];
        let y_diff: Vec<isize> = vec![2, -2, 2, -2, 1, -1, 1, -1];
        for i in 0..8 {
            knights.push(Point { x: self.x + x_diff[i], y: self.y + y_diff[i]});
        }

        knights
    }

    pub fn is_knight(&self, point: &Point) -> bool {
        let x_diff = (self.x - point.x).abs();
        let y_diff = (self.y - point.y).abs();

        (x_diff == 1 && y_diff == 2) || (x_diff == 2 && y_diff == 1)
    }
}

pub fn solve(point1: Point, point2: Point) -> bool {
    let knights = point1.get_all_knights();
    for point in knights.into_iter() {
        if point.is_knight(&point2) {
            return true;
        }
    }

    return false;
}

fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    }

    let point1 = Point::new(x1, y1);
    let point2 = Point::new(x2, y2);
    if solve(point1, point2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
