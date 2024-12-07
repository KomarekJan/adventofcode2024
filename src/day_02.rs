fn load_data() -> Vec<Vec<i32>> {
    let data = include_str!("../data/day_02.txt");
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        }).collect()
}


fn analyze_one_fail_accepted(data: &mut Vec<i32>) -> bool {
    for i in 0..data.len() {
        let mut temp = data.clone();
        temp.remove(i);
        if analyze_report(&mut temp) {
            return true;
        }
    }

    false
}

fn analyze_report(data: &mut Vec<i32>) -> bool {
    assert!(data.len() > 2);

    if data.get(0) > data.get(1) {
        data.reverse()
    }


    for (i, x) in data.iter().enumerate() {
        if let Some(y) = data.get(i + 1) {
            if y - x > 3 || y - x < 1 || y < x {
                return false;
            }
        }
    };
    true
}

#[allow(unused)]
pub fn solution() {
    let data = load_data();

    let result_01 = data.clone().iter_mut().fold(0, |acc, x| {
        if analyze_report(x) {
            acc + 1
        } else {
            acc
        }
    });

    let result_02 = data.clone().iter_mut().fold(0, |acc, x| {
        if analyze_one_fail_accepted(x) {
            acc + 1
        } else {
            acc
        }
    });

    println!("Day 02 Solution: {}", result_01);
    println!("Day 02 Solution: {}", result_02);
}