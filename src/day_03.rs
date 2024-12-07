struct Tokens<T> {
    data: Vec<T>,
    index: usize,
}

impl Tokens<u8> {
    fn new(data: Vec<u8>) -> Self {
        Tokens {
            data,
            index: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.index >= self.data.len()
    }

    fn peak_n(&self, n: usize) -> Option<Vec<u8>> {
        if self.index + n >= self.data.len() {
            return None;
        }

        let start = self.index;
        Some(self.data[start..start + n].to_vec())
    }

    fn read_token(&mut self) -> Option<u8> {
        if self.index >= self.data.len() {
            return None;
        }
        let x = self.data[self.index].clone();
        self.index += 1;
        Some(x)
    }

    fn peak_token(&self) -> Option<u8> {
        if self.index >= self.data.len() {
            return None;
        }
        Some(self.data[self.index].clone())
    }

    fn read_digit(&mut self) -> Option<u8> {
        if !self.peak_token().is_some_and(|x| (x as char).is_digit(10)) {
            return None;
        }

        if let Some(token) = self.read_token() {
            if (token as char).is_digit(10) {
                return Some(token);
            } else {}
        }

        None
    }

    fn get_number(&mut self) -> Option<u32> {
        let mut number = String::new();

        for _ in 0..3 {
            if let Some(digit) = self.read_digit() {
                number.push(digit as char);
            } else {
                break;
            }
        }
        if number.is_empty() {
            return None;
        }

        Some(number.parse().unwrap())
    }

    fn peak_string(&mut self, string: &String) -> bool {
        if let Some(peeked) = self.peak_n(string.len()) {
            let chars: Vec<char> = peeked.iter().map(|&x| x as char).collect();
            if chars == string.chars().collect::<Vec<char>>() {
                return true;
            }
        }
        false
    }

    fn get_mul(&mut self) -> bool {
        if self.peak_string(&"mul(".to_string()) {
            self.index += 4;
            return true;
        }
        false
    }

    fn get_sep(&mut self) -> bool {
        if self.peak_string(&",".to_string()) {
            self.index += 1;
            return true;
        }
        false
    }

    fn get_close(&mut self) -> bool {
        if self.peak_string(&")".to_string()) {
            self.index += 1;
            return true;
        }
        false
    }

    fn get_do(&mut self) -> bool {
        if self.peak_string(&"do(".to_string()) {
            self.index += 3;
            return true;
        }
        false
    }

    fn get_dont(&mut self) -> bool {
        if self.peak_string(&"don't(".to_string()) {
            self.index += 6;
            return true;
        }
        false
    }
}

fn load_data() -> String {
    include_str!("../data/day_03.txt").to_string()
}

#[allow(unused)]
pub fn solution() {
    let mut tokens = Tokens::new(load_data().into_bytes());
    let mut result = 0;
    let mut do_flag = true;

    while !tokens.is_empty() {
        let start = tokens.index;

        if tokens.get_do() {
            do_flag = true;
            println!("do");
            continue;
        }

        if tokens.get_dont() {
            do_flag = false;
            println!("don't");
            continue;
        }


        if !tokens.get_mul() {
            tokens.index += 1;
            continue;
        }

        println!("mul");
        let a = if let Some(number) = tokens.get_number() {
            number
        } else {
            continue;
        };

        if !tokens.get_sep() {
            continue;
        }

        let b = if let Some(number) = tokens.get_number() {
            number
        } else {
            continue;
        };

        if !tokens.get_close() {
            continue;
        }

        if do_flag {
            result += a * b;
        }
        let end = tokens.index;
        println!("Mul from string showed: {}", String::from_utf8_lossy(&tokens.data[start..end]));
    }

    println!("Result: {}", result);
}