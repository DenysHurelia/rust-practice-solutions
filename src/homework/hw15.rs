use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];

    let results: Vec<_> = digits
        .iter()
        .permutations(8)
        .unique()
        .par_bridge()
        .filter_map(|perm| {
            let m = *perm[0];
            let u = *perm[1];
            let x = *perm[2];
            let a = *perm[3];
            let s = *perm[4];
            let l = *perm[5];
            let o = *perm[6];
            let n = *perm[7];

            let muxa = m * 1000 + u * 100 + x * 10 + a;
            let slon = s * 1000 + l * 100 + o * 10 + n;

            if muxa * a == slon {
                Some(format!(
                    "{}\n  × {}\n------\n{}\n",
                    muxa, a, slon
                ))
            } else {
                None
            }
        })
        .collect();

    for solution in &results {
        println!("{}", solution);
    }

    println!("Кількість рішень: {}", results.len());
}
