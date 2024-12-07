use std::iter::zip;

fn load_data() -> (Vec<i32>, Vec<i32>) {
    let data = include_str!("../data/day_01.txt");
    data.lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a: i32 = parts.next().unwrap().parse().unwrap();
            let b: i32 = parts.next().unwrap().parse().unwrap();
            (a, b)
        }).unzip()
}

#[allow(unused)]
pub(crate) fn solution() {
    let mut data = load_data();
    let first = data.0.as_mut_slice();
    let second = data.1.as_mut_slice();

    first.sort();
    second.sort();

    let result = zip(first.iter(), second.iter()).fold(0, |acc, (a, b)| {
        acc + if a > b {
            a - b
        } else {
            b - a
        }
    });

    println!("Day 01 Solution: {}", result);

    let mut total_similarity_score = 0;
    let mut b_position = 0;
    for a in first {
        while b_position < second.len() && second[b_position] < *a {
            b_position += 1;
        }

        if b_position >= second.len() {
            break;
        }

        let mut appears = 0;
        while second[b_position] == *a {
            appears += 1;
            b_position += 1;
        }
        total_similarity_score += *a * appears;
    }

    println!("Day 01 Solution: {}", total_similarity_score);
}