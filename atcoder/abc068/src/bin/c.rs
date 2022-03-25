use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ships: [(usize, usize); m],
    }

    // whether can land on i from 1.
    let mut can_land_to_i = vec![false; n + 1];
    // whether can land on n from i.
    let mut can_land_from_i = vec![false; n + 1];

    for (a, b) in ships {
        // if one is 1, we can land on the other island.
        if a == 1 {
            can_land_to_i[b] = true;
        }
        if b == 1 {
            can_land_to_i[a] = true;
        }
        // if one is n, we can land from the other.
        if a == n {
            can_land_from_i[b] = true;
        }
        if b == n {
            can_land_from_i[a] = true;
        }
    }

    for i in 1..=n {
        if can_land_from_i[i] && can_land_to_i[i] {
            println!("POSSIBLE");
            return;
        }
    }

    println!("IMPOSSIBLE");
}
