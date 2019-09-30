pub fn read_count(promt_str: String) -> u32 {
    let mut input_line = String::new();
    println!("{}", promt_str);
    loop {
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to readline.");
        let count: u32 = match input_line.trim_end().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input positive integer.");
                continue;
            }
        };
        return count;
    }
}

#[allow(dead_code)]
pub fn read_character(promt_str: String) -> char {
    let mut input_line: String = String::new();
    println!("{}", promt_str);
    loop {
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line.");
        if input_line.trim_end().len() == 1 {
            let x = input_line.chars().next().unwrap();
            return x;
        } else {
            println!("Please input one character.");
            continue;
        }
    }
}

pub fn read_characters(promt_str: String) -> Vec<char> {
    let mut input_line: String = String::new();
    let mut characters = Vec::new();
    println!("{}", promt_str);
    loop {
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line.");
        let trimed_chars = input_line.trim_end().replace(" ", "");
        let count = trimed_chars.len();
        if count < 1 {
            println!("Please input at least one character.");
            continue;
        }
        for i in 0..(count - 1) {
            characters.push(trimed_chars.chars().nth(i).unwrap());
            return characters;
        }
    }
}