use std::{fmt::Display, path::Path, time::Instant};

const YEAR: usize = 2022;

pub trait Parser: Sized + Challenge {
    fn parse(input: &'static str) -> nom::IResult<&'static str, Self>;
}

pub trait Challenge {
    const NAME: &'static str;

    type Output1: Display;
    fn part_one(self) -> Self::Output1;

    type Output2: Display;
    fn part_two(self) -> Self::Output2;
}

pub fn check<C: Parser + Clone>(input: &'static str) {
    let start = Instant::now();
    let challenge = C::parse(input).unwrap().1;
    let p1 = challenge.clone().part_one();
    let p2 = challenge.part_two();
    println!("took: {:?}", start.elapsed());

    println!("\tAnswer to part one: {p1}");
    println!("\tAnswer to part two: {p2}");
}

pub fn run<C: Parser>(input: &'static str) {
    println!("\nRunning challenge {}", C::NAME);

    let start = Instant::now();
    let challenge = C::parse(input).unwrap().1;

    let file = Path::new("challenges").join(C::NAME).join("README.md");
    let readme = std::fs::read_to_string(file).expect("could not read file");
    let part_one = !readme.contains("--- Part Two ---");

    if part_one {
        let p1 = challenge.part_one();
        println!("took: {:?}", start.elapsed());
        println!("\tAnswer to part one: {p1}. ({:?})", start.elapsed());
        submit::<C, _>(1, p1);
    } else {
        let p2 = challenge.part_two();
        println!("\tAnswer to part two: {p2}. ({:?})", start.elapsed());
        submit::<C, _>(2, p2);
    }
}

fn submit<C: Challenge, S: Display>(level: usize, answer: S) {
    let session = dotenv::var("AOC_SESSION").unwrap();

    let day = C::NAME[3..].parse::<i32>().unwrap();
    let url = format!("https://adventofcode.com/{YEAR}/day/{day}/answer");

    ureq::post(&url)
        .set("Cookie", &format!("session={session}"))
        .send_form(&[("level", &format!("{level}")), ("answer", &format!("{answer}"))])
        .unwrap();
}
