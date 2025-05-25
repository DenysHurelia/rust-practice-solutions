const H: usize = 21;
const W: usize = 21;

fn main() {
    let mid = H / 2;

    for y in 0..H {
        let mut row = String::new();

        let dy = if y <= mid { y } else { H - 1 - y };

        let stars = 1 + 2 * dy;
        let spaces = (W - stars) / 2;

        for x in 0..W {
            let ch = if x < spaces || x >= spaces + stars { ' ' } else { '*' };
            row.push(ch);
        }

        println!("{}", row);
    }
}
