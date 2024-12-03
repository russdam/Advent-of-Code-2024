fn is_safe<'a>(mut numbers: impl Iterator<Item = &'a isize>) -> bool {
    let mut ascending = None;
    let mut a = numbers.next().unwrap();
    for b in numbers {
        if a == b || a.abs_diff(*b) > 3 {
            return false;
        }

        match ascending {
            None => ascending = Some(a > b),
            Some(true) => {
                if a < b {
                    return false;
                }
            }
            Some(false) => {
                if a > b {
                    return false;
                }
            }
        }

        a = b;
    }

    true
}

fn main() {
    let input = include_str!("../data/data-1.in");

    let _input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    let num_ok_reports = input
        .lines()
        .filter(|line| {
            let numbers: Vec<_> = line
                .split_ascii_whitespace()
                .map(|piece| piece.parse::<isize>().unwrap())
                .collect();

            is_safe(numbers.iter())
        })
        .count();

    println!("part_a => {num_ok_reports}");

    let num_ok_reports = input
        .lines()
        .filter(|line| {
            let numbers: Vec<_> = line
                .split_ascii_whitespace()
                .map(|piece| piece.parse::<isize>().unwrap())
                .collect();

            if is_safe(numbers.iter()) {
                return true;
            }

            for i in 0..numbers.len() {
                let iter = numbers[..i].iter().chain(numbers[(i + 1)..].iter());
                if is_safe(iter) {
                    return true;
                }
            }

            false
        })
        .count();

    println!("part_b => {num_ok_reports}");
}
