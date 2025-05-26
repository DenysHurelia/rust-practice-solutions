fn main() {
    let triangles = 6;
    draw_tree(triangles);
}

fn draw_tree(triangles: usize) {
    let max_width = 2 * triangles + 1 + 2 * (triangles - 1);

    (0..triangles).for_each(|t| {
        let height = t + 2;

        (0..height).for_each(|i| {
            let stars = 1 + 2 * i;
            let spaces = (max_width - stars) / 2;

            let line: String = std::iter::repeat(" ").take(spaces).collect::<String>()
                + &std::iter::repeat("*").take(stars).collect::<String>();

            println!("{}", line);
        });
    });
}
