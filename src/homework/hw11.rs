use rand::Rng;

fn main() {
    let vec = gen_random_vector(20);
    let (idx1, idx2, sum) = min_adjacent_sum(&vec);
    print_vector_with_min_pair(&vec, idx1, idx2, sum);
}

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    assert!(data.len() >= 2, "Vector length must be at least 2");

    data.windows(2)
        .enumerate()
        .min_by_key(|&(_, pair)| pair[0] + pair[1])
        .map(|(idx, pair)| (idx, idx + 1, pair[0] + pair[1]))
        .unwrap()
}

fn print_vector_with_min_pair(data: &[i32], idx1: usize, idx2: usize, sum: i32) {
    print!("indexes:");
    for i in 0..data.len() {
        print!(" {:>2}.", i);
    }
    println!();

    print!("data:   [");
    for (i, val) in data.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{:2}", val);
    }
    println!("]");

    print!("indexes: ");
    for i in 0..data.len() {
        if i == idx1 {
            print!("\\__");
        } else if i == idx2 {
            print!(" __/");
        } else {
            print!("   ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[idx1], data[idx2], sum, idx1, idx2
    );
}
