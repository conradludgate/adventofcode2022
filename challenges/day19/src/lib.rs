use aoc::{Challenge, Parser as ChallengeParser};
use nom::{bytes::complete::tag, IResult, Parser};
use pathfinding::prelude::dijkstra;

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
                obsidian: (
                    obsidian_cost_ore.parse().unwrap(),
                    obsidian_cost_clay.parse().unwrap(),
                ),
                geode: (
                    geode_cost_ore.parse().unwrap(),
                    geode_cost_obsidian.parse().unwrap(),
                ),
            });
        }

        Ok(("", Self(bp)))
    }
}

impl Blueprint {
    fn solve(self) -> usize {
        #[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
        struct State {
            time: usize,
            ore: usize,
            clay: usize,
            obsidian: usize,
            geodes: usize,
            ore_robots: usize,
            clay_robots: usize,
            obsidian_robots: usize,
            geode_robots: usize,
        }

        let res = dijkstra(
            &State {
                time: 0,
                ore: 0,
                clay: 0,
                obsidian: 0,
                geodes: 0,
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            },
            |&s| {
                // let base = State {
                //     time: s.time + 1,
                //     ore: s.ore + s.ore_robots,
                //     clay: s.clay + s.clay_robots,
                //     obsidian: s.obsidian + s.obsidian_robots,
                //     geodes: s.geodes + s.geode_robots,
                //     ..*s
                // };

                // let cost = 24 - s.geode_robots;

                // build a robot now
                // needs to be lazy to avoid underflows
                #[allow(clippy::unnecessary_lazy_evaluations)]
                let build_robots = [
                    (s.ore >= self.ore).then(|| State {
                        ore_robots: s.ore_robots + 1,
                        ore: s.ore - self.ore,
                        ..s
                    }),
                    (s.ore >= self.clay).then(|| State {
                        clay_robots: s.clay_robots + 1,
                        ore: s.ore - self.clay,
                        ..s
                    }),
                    (s.ore >= self.obsidian.0 && s.clay >= self.obsidian.1).then(|| State {
                        obsidian_robots: s.obsidian_robots + 1,
                        ore: s.ore - self.obsidian.0,
                        clay: s.clay - self.obsidian.1,
                        ..s
                    }),
                    (s.ore >= self.geode.0 && s.obsidian >= self.geode.1).then(|| State {
                        geode_robots: s.geode_robots + 1,
                        ore: s.ore - self.geode.0,
                        obsidian: s.obsidian - self.geode.1,
                        ..s
                    }),
                ];

                // fast forward to building a robot
                #[allow(clippy::unnecessary_lazy_evaluations)]
                let ff_build_robots = [
                    (s.ore < self.ore).then(|| State {
                        ore_robots: s.ore_robots + 1,
                        ore: s.ore.wrapping_sub(self.ore),
                        time: s.time + (self.ore - s.ore) / s.ore_robots,
                        ..s
                    }),
                    (s.ore < self.clay).then(|| State {
                        clay_robots: s.clay_robots + 1,
                        ore: s.ore.wrapping_sub(self.clay),
                        time: s.time + (self.clay - s.ore) / s.ore_robots,
                        ..s
                    }),
                    (s.ore < self.obsidian.0 && s.clay >= self.obsidian.1).then(|| State {
                        obsidian_robots: s.obsidian_robots + 1,
                        ore: s.ore.wrapping_sub(self.obsidian.0),
                        clay: s.clay.wrapping_sub(self.obsidian.1),
                        time: s.time + (self.obsidian.0 - s.ore) / s.ore_robots,
                        ..s
                    }),
                    (s.ore >= self.obsidian.0 && s.clay < self.obsidian.1 && s.clay_robots > 0)
                        .then(|| State {
                            obsidian_robots: s.obsidian_robots + 1,
                            ore: s.ore.wrapping_sub(self.obsidian.0),
                            clay: s.clay.wrapping_sub(self.obsidian.1),
                            time: s.time + (self.obsidian.1 - s.clay) / s.clay_robots,
                            ..s
                        }),
                    (s.ore < self.obsidian.0 && s.clay < self.obsidian.1 && s.clay_robots > 0)
                        .then(|| State {
                            obsidian_robots: s.obsidian_robots + 1,
                            ore: s.ore.wrapping_sub(self.obsidian.0),
                            clay: s.clay.wrapping_sub(self.obsidian.1),
                            time: s.time
                                + usize::max(
                                    (self.obsidian.0 - s.ore) / s.ore_robots,
                                    (self.obsidian.1 - s.clay) / s.clay_robots,
                                ),
                            ..s
                        }),
                    (s.ore < self.geode.0 && s.obsidian >= self.geode.1).then(|| State {
                        geode_robots: s.geode_robots + 1,
                        ore: s.ore.wrapping_sub(self.geode.0),
                        obsidian: s.obsidian.wrapping_sub(self.geode.1),
                        time: s.time + (self.geode.0 - s.ore) / s.ore_robots,
                        ..s
                    }),
                    (s.ore >= self.geode.0 && s.obsidian < self.geode.1 && s.obsidian_robots > 0)
                        .then(|| State {
                            geode_robots: s.geode_robots + 1,
                            ore: s.ore.wrapping_sub(self.geode.0),
                            obsidian: s.obsidian.wrapping_sub(self.geode.1),
                            time: s.time + (self.geode.1 - s.obsidian) / s.obsidian_robots,
                            ..s
                        }),
                    (s.ore < self.geode.0 && s.obsidian < self.geode.1 && s.obsidian_robots > 0)
                        .then(|| State {
                            geode_robots: s.geode_robots + 1,
                            ore: s.ore.wrapping_sub(self.geode.0),
                            obsidian: s.obsidian.wrapping_sub(self.geode.1),
                            time: s.time
                                + usize::max(
                                    (self.geode.0 - s.ore) / s.ore_robots,
                                    (self.geode.1 - s.obsidian) / s.obsidian_robots,
                                ),
                            ..s
                        }),
                ];

                build_robots
                    .into_iter()
                    .chain(ff_build_robots)
                    .flatten()
                    .map(move |s1| {
                        let t = s1.time - s.time + 1;
                        (
                            State {
                                time: s1.time + 1,
                                ore: s.ore.wrapping_add(s.ore_robots * t),
                                clay: s.clay.wrapping_add(s.clay_robots * t),
                                obsidian: s.obsidian.wrapping_add(s.obsidian_robots * t),
                                geodes: s.geodes.wrapping_add(s.geode_robots * t),
                                ..s1
                            },
                            (24 - s.geode_robots) * t,
                        )
                    })
            },
            |s| s.time == 23,
        )
        .unwrap();

        let mut count = 0;
        for i in res.0 {
            dbg!(i);
            count += i.geode_robots;
        }
        dbg!(count);

        // 24 * 24 - dbg!(res.1)
        count
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        let mut sum = 0;
        for (i, bp) in self.0.into_iter().enumerate() {
            sum += (i + 1) * bp.solve()
        }
        sum
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
        assert_eq!(output.part_one(), 33);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 0);
    }
}
