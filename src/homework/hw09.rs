fn main() {
    let s = String::from("abcdefgh");
    let rotated = rotate(s.clone(), 2);
    println!("Original: {}", s);
    println!("Rotated:  {}", rotated);
}

fn rotate(s: String, n: isize) -> String {
    let len = s.len();

    if len == 0 {
        return s;
    }

    let n = n.rem_euclid(len as isize) as usize;

    let split_index = len - n;
    let (left, right) = s.split_at(split_index);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, expected)| {
            assert_eq!(rotate(s.clone(), *n), expected.to_string());
        });
    }
}
