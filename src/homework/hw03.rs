use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter width (10..80):");
    io::stdin().read_line(&mut input).unwrap();
    let w: usize = input.trim().parse().unwrap();

    input.clear();
    println!("Enter height (10..80):");
    io::stdin().read_line(&mut input).unwrap();
    let h: usize = input.trim().parse().unwrap();

    draw_envelope(w, h);
}

fn draw_envelope(w: usize, h: usize) {
    for y in 0..h {
        let mut row = String::new();

        for x in 0..w {
            let ch = if y == 0 || y == h - 1 {
                '*'
            } else if x == 0 || x == w - 1 {
                '*'
            } else if x == y * (w - 1) / (h - 1) || x == (w - 1) - y * (w - 1) / (h - 1) {
                '*'
            } else {
                ' '
            };

            row.push(ch);
        }

        println!("{}", row);
    }
}