use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut points = HashSet::new();

    for rect in xs {
        for x in rect.a.x..rect.b.x {
            for y in rect.a.y..rect.b.y {
                points.insert(Point { x, y });
            }
        }
    }

    points.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 0, y: 0 },
            b: Point { x: 6, y: 3 },
        },
        Rectangle {
            a: Point { x: 5, y: 1 },
            b: Point { x: 15, y: 3 }, 
        },
        Rectangle {
            a: Point { x: 0, y: 3 },
            b: Point { x: 4, y: 11 }, 
        },
    ]
}

fn main() {
    let rects = test_data();
    let occupied_area = area_occupied(&rects);
    println!("Загальна зайнята площа: {}", occupied_area);
}
