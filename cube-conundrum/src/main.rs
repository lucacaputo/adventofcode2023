//TODO: split into modules, will ya? (no I won't)

extern crate regex;

use regex::Regex;
use std::{env, fs};

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct Extraction {
    r: u32,
    g: u32,
    b: u32,
}

impl Extraction {
    fn is_valid(&self, r: u32, g: u32, b: u32) -> bool {
        self.r <= r && self.g <= g && self.b <= b
    }

    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

//TODO: implement FromStr instead
impl From<&str> for Extraction {
    fn from(value: &str) -> Self {
        let r = get_u32("r", value);
        let g = get_u32("g", value);
        let b = get_u32("b", value);

        Self { r, g, b }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    extractions: Vec<Extraction>,
}

impl Game {
    pub fn minimum_set(&self) -> Result<Extraction, &str> {
        let (min_red, min_green, min_blue) = (
            self.extractions.iter().map(|extraction| extraction.r).max(),
            self.extractions.iter().map(|extraction| extraction.g).max(),
            self.extractions.iter().map(|extraction| extraction.b).max(),
        );
        match (min_red, min_green, min_blue) {
            (Some(r), Some(g), Some(b)) => Ok(Extraction { r, g, b }),
            _ => Err("something went wrong"),
        }
    }
}

//(?:(?:(?<r>\d+) red)|(?:(?<g>\d+) green)|(?:(?<b>\d+) blue))
//pd, only works with captures_iter

lazy_static! {
    static ref R_REGEX: Regex = Regex::new(r"(?<r>\d+) red").unwrap();
    static ref G_REGEX: Regex = Regex::new(r"(?<g>\d+) green").unwrap();
    static ref B_REGEX: Regex = Regex::new(r"(?<b>\d+) blue").unwrap();
}

fn get_u32(group: &str, haystack: &str) -> u32 {
    let exp: &Regex = match group {
        "r" => &R_REGEX,
        "g" => &G_REGEX,
        "b" => &B_REGEX,
        _ => panic!("invalid group"),
    };
    exp.captures(haystack)
        .and_then(|v| {
            Some(
                v.name(group)
                    .map_or("0", |val| val.as_str())
                    .parse::<u32>()
                    .unwrap(),
            )
        })
        .unwrap_or(0)
}

fn parse_line(line: &str) -> Game {
    let pieces: Vec<_> = line.split(':').collect();
    let (game_id, extractions) = (
        pieces
            .get(0)
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap_or(0),
        pieces
            .get(1)
            .unwrap()
            .split(';')
            .map(Extraction::from)
            .collect::<Vec<Extraction>>(),
    );
    Game {
        id: game_id,
        extractions,
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("invalid path");
    let lines: Vec<_> = contents.split('\n').collect();

    let games: Vec<Game> = lines.into_iter().map(parse_line).collect();
    let valid_game_ids: Vec<_> = games
        .iter()
        .filter(|&game| {
            game.extractions
                .iter()
                .all(|extraction| extraction.is_valid(12, 13, 14))
        })
        .map(|game| game.id)
        .collect();

    let sum: u32 = valid_game_ids.into_iter().sum();

    println!("sum of ids is {}!", sum);

    let minimum_sets = games
        .iter()
        .map(|game| game.minimum_set().unwrap().power())
        .sum::<u32>();

    println!("minimum sets added up are {}!", minimum_sets);
}
