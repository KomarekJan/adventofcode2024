fn load_data() -> Vec<Vec<char>> {
    let data: Vec<Vec<u8>> = include_str!("../data/day_04.txt").split_whitespace().map(|x| x.as_bytes().to_vec()).collect();
    let data: Vec<Vec<char>> = data.iter().map(|x| x.iter().map(|&x| x as char).collect()).collect();

    data
}

fn get_char(data: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<char> {
    Some(data.get(pos.1)?.get(pos.0)?.clone())
}

fn check_left(data: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<bool> {
    if pos.0 < 3 {
        return None;
    }

    Some(true
        && get_char(data, (pos.0, pos.1))? == 'X'
        && get_char(data, (pos.0 - 1, pos.1))? == 'M'
        && get_char(data, (pos.0 - 2, pos.1))? == 'A'
        && get_char(data, (pos.0 - 3, pos.1))? == 'S'
    )
}

fn check_right(data: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<bool> {
    Some(true
        && get_char(data, (pos.0, pos.1))? == 'X'
        && get_char(data, (pos.0 + 1, pos.1))? == 'M'
        && get_char(data, (pos.0 + 2, pos.1))? == 'A'
        && get_char(data, (pos.0 + 3, pos.1))? == 'S'
    )
}

fn check_up(data: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<bool> {
    if pos.1 < 3 {
        return None;
    }

    Some(true
        && get_char(data, (pos.0, pos.1))? == 'X'
        && get_char(data, (pos.0, pos.1 - 1))? == 'M'
        && get_char(data, (pos.0, pos.1 - 2))? == 'A'
        && get_char(data, (pos.0, pos.1 - 3))? == 'S'
    )
}

fn check_down(data: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<bool> {
    Some(true
        && get_char(data, (pos.0, pos.1))? == 'X'
        && get_char(data, (pos.0, pos.1 + 1))? == 'M'
        && get_char(data, (pos.0, pos.1 + 2))? == 'A'
        && get_char(data, (pos.0, pos.1 + 3))? == 'S'
    )
}

fn check_diag_up_left(data: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<bool> {
    if pos.0 < 3 || pos.1 < 3 {
        return None;
    }

    Some(true
        && get_char(data, (pos.0, pos.1))? == 'X'
        && get_char(data, (pos.0 - 1, pos.1 - 1))? == 'M'
        && get_char(data, (pos.0 - 2, pos.1 - 2))? == 'A'
        && get_char(data, (pos.0 - 3, pos.1 - 3))? == 'S'
    )
}

fn check_diag_up_right(data: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<bool> {
    if pos.1 < 3 {
        return None;
    }

    Some(true
        && get_char(data, (pos.0, pos.1))? == 'X'
        && get_char(data, (pos.0 + 1, pos.1 - 1))? == 'M'
        && get_char(data, (pos.0 + 2, pos.1 - 2))? == 'A'
        && get_char(data, (pos.0 + 3, pos.1 - 3))? == 'S'
    )
}

fn check_diag_down_left(data: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<bool> {
    if pos.0 < 3 {
        return None;
    }

    Some(true
        && get_char(data, (pos.0, pos.1))? == 'X'
        && get_char(data, (pos.0 - 1, pos.1 + 1))? == 'M'
        && get_char(data, (pos.0 - 2, pos.1 + 2))? == 'A'
        && get_char(data, (pos.0 - 3, pos.1 + 3))? == 'S'
    )
}

fn check_diag_down_right(data: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<bool> {
    Some(true
        && get_char(data, (pos.0, pos.1))? == 'X'
        && get_char(data, (pos.0 + 1, pos.1 + 1))? == 'M'
        && get_char(data, (pos.0 + 2, pos.1 + 2))? == 'A'
        && get_char(data, (pos.0 + 3, pos.1 + 3))? == 'S'
    )
}


fn check_xmas(data: &Vec<Vec<char>>, pos: (usize, usize)) -> usize {
    let checks = vec![
        check_left,
        check_right,
        check_up,
        check_down,
        check_diag_up_left,
        check_diag_up_right,
        check_diag_down_left,
        check_diag_down_right,
    ];

    let mut count = 0;
    for check in checks {
        if check(data, pos).is_some_and(|x| x) {
            count += 1;
        }
    }
    count
}

fn check_x_mas(data: &Vec<Vec<char>>, pos: (usize, usize)) -> usize {
    let patterns = vec![
        ('M', 'S', 'A', 'M', 'S'),
        ('S', 'M', 'A', 'S', 'M'),
        ('S', 'S', 'A', 'M', 'M'),
        ('M', 'M', 'A', 'S', 'S'),
    ];

    let mut count = 0;

    for (x, y, z, a, b) in patterns {
        if pos.0 < 1 || pos.1 < 1 {
            continue;
        }

        if true
            && Some(x) == get_char(data, (pos.0 - 1, pos.1 - 1))
            && Some(y) == get_char(data, (pos.0 + 1, pos.1 - 1))
            && Some(z) == get_char(data, (pos.0, pos.1))
            && Some(a) == get_char(data, (pos.0 - 1, pos.1 + 1))
            && Some(b) == get_char(data, (pos.0 + 1, pos.1 + 1)) {
            count += 1;
        }
    }

    count
}

#[allow(unused)]
pub fn solution() {
    let data = load_data();
    let mut counter = 0;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            counter += check_xmas(&data, (x, y));
        }
    }

    println!("Total xmas found: {}", counter);

    let mut counter = 0;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            counter += check_x_mas(&data, (x, y));
        }
    }

    println!("Total x-mas found: {}", counter);
}