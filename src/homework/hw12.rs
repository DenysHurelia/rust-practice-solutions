fn main() {
    let shipments1 = vec![8, 2, 2, 4, 4];
    let shipments2 = vec![9, 3, 7, 2, 9];

    println!("Shipments1: {:?}, moves needed: {}", shipments1, count_permutation(&shipments1));
    println!("Shipments2: {:?}, moves needed: {}", shipments2, count_permutation(&shipments2));

    let n = 5;
    let generated = gen_shipments(n);
    println!("Generated shipments (length {}): {:?}", n, generated);
    println!("Moves needed for generated: {}", count_permutation(&generated));
}

fn can_distribute_equally(shipments: &Vec<u32>) -> bool {
    let total: u32 = shipments.iter().sum();
    total % (shipments.len() as u32) == 0
}

fn count_permutation(shipments: &Vec<u32>) -> usize {
    if !can_distribute_equally(shipments) {
        return 0;
    }

    let n = shipments.len() as u32;
    let total: u32 = shipments.iter().sum();
    let average = total / n;

    shipments.iter()
        .map(|&x| if x > average { (x - average) as usize } else { 0 })
        .sum()
}

fn gen_shipments(n: usize) -> Vec<u32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut v: Vec<u32> = (0..n).map(|_| rng.gen_range(10..100)).collect();

    let sum: u32 = v.iter().sum();
    let remainder = sum % (n as u32);

    if remainder != 0 {
        let diff = (n as u32) - remainder;
        v[0] += diff;
    }

    v
}
