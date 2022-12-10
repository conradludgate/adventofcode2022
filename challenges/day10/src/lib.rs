use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayString;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(i32, ArrayString<8>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut x = 1;
        let mut cycle: usize = 1;
        let mut output = [b'.'; 40 * 6]; // interpreted as 5*8*6 => 30*8
        let mut total = 0;
        for line in input.lines() {
            let (cycles, newx) = match &line[..4] {
                "addx" => (2, line[5..].parse::<i32>().unwrap() + x),
                _ => (1, x),
            };

            for _ in 0..cycles {
                // if center of screen, track signal strength
                if (cycle + 20) % 40 == 0 {
                    total += x * cycle as i32;
                }

                let row = (cycle - 1) % 40;
                let col = (cycle - 1) / 40;
                let sprite = x - 1..=x + 1;
                if sprite.contains(&(row as i32)) {
                    output[col + row * 6] = b'#';
                }

                cycle += 1;
            }

            x = newx;
        }

        let mut s = ArrayString::<8>::new();

        for line in output.chunks(30) {
            let c = match line {
                b".######..#..#..#...#####......" => 'A',
                b"#######.#..##.#..#.#.##......." => 'B',
                b".####.#....##....#.#..#......." => 'C',
                b"#######....##....#.####......." => 'D',
                b"#######.#..##.#..##....#......" => 'E',
                b"#######.#...#.#...#..........." => 'F',
                b".####.#....##..#.#.#.###......" => 'G',
                b"######..#.....#...######......" => 'H',
                b"#....########....#............" => 'I',
                b"....#......##....######......." => 'J',
                b"######..#....#.##.#....#......" => 'K',
                b"######.....#.....#.....#......" => 'L',
                b"######.##....##...######......" => 'M',
                b"######.##......##.######......" => 'N',
                b".####.#....##....#.####......." => 'O',
                b"#######..#..#..#...##........." => 'P',
                b".####.#...###....#.####......." => 'Q',
                b"#######..#..#..##..##..#......" => 'R',
                b".#..#.#.#..##..#.#.#..#......." => 'S',
                b"#.....#######................." => 'T',
                b"#####......#.....######......." => 'U',
                b"####......##....######........" => 'V',
                b"######...##....##.######......" => 'W',
                b"##..##..##....##..##..##......" => 'X',
                b"###......######..............." => 'Y',
                b"#...###..#.##.#..###...#......" => 'Z',
                _ => '.',
            };
            s.push(c);
        }

        Ok(("", Self(total, s)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = i32;
    fn part_one(self) -> Self::Output1 {
        self.0
    }

    type Output2 = ArrayString<8>;
    fn part_two(self) -> Self::Output2 {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 13140);
    }
}
