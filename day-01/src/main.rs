#![allow(unused_variables)]

fn main() {
    let exampledata_1 = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    let data_1 = include_str!("../data/data-1.in");

    let exampledata_2 = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    let data_2 = include_str!("../data/data-2.in");

    part_one(data_1);
    part_two(data_1);
}

fn split_data(inp: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for l in inp.lines() {
        let mut vals = l.split_ascii_whitespace();
        left.push(vals.next().unwrap().parse::<u32>().unwrap());
        right.push(vals.next().unwrap().parse::<u32>().unwrap());
    }
    (left, right)
}

fn count(num: u32, vector: &[u32]) -> u32 {
    vector
        .iter()
        .fold(0u32, |a, v| if *v == num { a + 1 } else { a })
}

fn part_one(data: &str) {
    let (mut left, mut right) = split_data(data);

    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        let apart = left[i].abs_diff(right[i]);
        sum += apart;
    }
    println!("Total is {sum}");
}

fn part_two(data: &str) {
    let (mut left, mut right) = split_data(data);

    left.sort();
    right.sort();

    let sum = left
        .into_iter()
        .fold(0, |acc, val| acc + val * count(val, &right));

    println!("Total is {sum}");
}
