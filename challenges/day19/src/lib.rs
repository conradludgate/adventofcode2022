use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, IResult, Parser};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Blueprint {
    // cost in ore
    ore: usize,
    // cost in ore
    clay: usize,
    // cost in (ore, clay)
    obsidian: (usize, usize),
    // cost in (ore, obsidian)
    geode: (usize, usize),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<Blueprint>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut bp = vec![];
        for line in input.lines() {
            let Some((_, line)) = line.split_once("ore robot costs ") else { continue };
            let Some((ore_cost, line)) = line.split_once(" ore. Each clay robot costs ") else { continue };
            let Some((clay_cost, line)) = line.split_once(" ore. Each obsidian robot costs ") else { continue };
            let Some((obsidian_cost, line)) = line.split_once(" clay. Each geode robot costs ") else { continue };
            let Some((geode_cost, _)) = line.split_once(" obsidian.") else { continue };

            let Some((obsidian_cost_ore, obsidian_cost_clay)) = obsidian_cost.split_once(" ore and ") else { continue };
            let Some((geode_cost_ore, geode_cost_obsidian)) = geode_cost.split_once(" ore and ") else { continue };

            bp.push(Blueprint {
                ore: ore_cost.parse().unwrap(),
                clay: clay_cost.parse().unwrap(),
                obsidian: (obsidian_cost_ore.parse().unwrap(), obsidian_cost_clay.parse().unwrap()),
                geode: (geode_cost_ore.parse().unwrap(), geode_cost_obsidian.parse().unwrap()),
            });
        }

        Ok(("", Self(bp)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        0
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 0);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
