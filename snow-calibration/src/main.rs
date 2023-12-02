use std::{env, fs};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref NUMBERS_IN_LETTERS: [(&'static str, &'static str); 9] = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("invalid path");
    let lines: Vec<_> = contents.split('\n').collect();

    let numbers_only_rows: Vec<_> = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .fold(String::new(), |acc, curr| {
                    let mut current = format!("{}{}", acc, curr);
                    NUMBERS_IN_LETTERS.iter().for_each(|&(num, ch)| {
                        if current.ends_with(num) {
                            current = current.replace(num, format!("{}{}", ch, num).as_str());
                        }
                    });
                    current
                })
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<char>>()
        })
        .collect();

    let pairs: Vec<_> = numbers_only_rows
        .into_iter()
        .map(|numbers| match numbers[..] {
            [first] => [first, first],
            [first, .., last] => [first, last],
            _ => panic!("invalid slice"),
        })
        .map(|[first, second]| format!("{}{}", first, second).parse::<u32>().unwrap())
        .collect();

    let sum: u32 = pairs.into_iter().sum();

    println!("sum is {}!", sum)
}
