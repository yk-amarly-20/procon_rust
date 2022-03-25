use std::{vec};

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
    }

    // large enough vector
    // each vector, whether we can pour up i water or sugar
    let mut dpwater = vec![0; 50];
    let mut dpsugar = vec![0; 4000];

    dpwater[a] = 1;
    dpwater[b] = 1;
    dpsugar[c] = 1;
    dpsugar[d] = 1;

    for i in 0..40 {
        if i + a < 40 {
            dpwater[i + a] = dpwater[i + a].max(dpwater[i]);
        }

        if i + b < 40 {
            dpwater[i + b] = dpwater[i + b].max(dpwater[i]);
        }
    }

    for j in 0..4000 {
        if j + c < 4000 {
            dpsugar[j + c] = dpsugar[j + c].max(dpsugar[j]);
        }

        if j + d < 4000 {
            dpsugar[j + d] = dpsugar[j + d].max(dpsugar[j]);
        }
    }

    let mut water = 100. * a as f64;
    let mut sugar = 0.;

    for i in 0..40 {
        for j in 0..4000 {
            if dpwater[i] > 0 && dpsugar[j] > 0 && 100 * i + j <= f && i * e >= j {
                // compare concentration
                if sugar / (water + sugar) < (j as f64) / (100. * i as f64 + j as f64) {
                    water = 100. * i as f64;
                    sugar = j as f64;
                }
            }
        }
    }

    println!("{} {}", water + sugar, sugar);
}
