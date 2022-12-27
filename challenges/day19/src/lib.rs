use std::{num::NonZeroU16, ops};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::IResult;
use pathfinding::prelude::astar;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Blueprint {
    ore: Vector,
    clay: Vector,
    obsidian: Vector,
    geode: Vector,
}

#[derive(Debug, PartialEq, Clone, Copy, Default, Hash, Eq)]
struct Vector {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}
impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}
impl ops::Mul<u16> for Vector {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self::Output {
        Self {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
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
                ore: Vector {
                    ore: ore_cost.parse().unwrap(),
                    ..<_>::default()
                },
                clay: Vector {
                    ore: clay_cost.parse().unwrap(),
                    ..<_>::default()
                },
                obsidian: Vector {
                    ore: obsidian_cost_ore.parse().unwrap(),
                    clay: obsidian_cost_clay.parse().unwrap(),
                    ..<_>::default()
                },
                geode: Vector {
                    ore: geode_cost_ore.parse().unwrap(),
                    obsidian: geode_cost_obsidian.parse().unwrap(),
                    ..<_>::default()
                },
            });
        }

        Ok(("", Self(bp)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct State {
    time: u16,
    store: Vector,
    production: Vector,
}

impl State {
    /// what's the minimum time we should wait until we can afford the cost and build the robot
    fn wait_for(&self, cost: Vector, until: u16) -> Option<u16> {
        fn wait_for(current: u16, cost: u16, production_rate: u16) -> Option<u16> {
            let remaining = cost.saturating_sub(current);
            if remaining > 0 {
                NonZeroU16::new(production_rate).map(|n| (remaining + n.get() - 1) / n)
            } else {
                Some(0)
            }
        }

        let ore = wait_for(self.store.ore, cost.ore, self.production.ore)?;
        let clay = wait_for(self.store.clay, cost.clay, self.production.clay)?;
        let obsidian = wait_for(self.store.obsidian, cost.obsidian, self.production.obsidian)?;
        let max_wait = u16::max(ore, u16::max(clay, obsidian)) + 1;
        (self.time + max_wait <= until).then_some(max_wait)
    }

    /// return the next state in which we build a robot
    fn build(&self, cost: Vector, produces: Vector, until: u16) -> Option<(Self, u16)> {
        self.wait_for(cost, until).map(|time_waiting| {
            (
                State {
                    time: self.time + time_waiting,
                    store: self.store + self.production * time_waiting - cost,
                    production: self.production + produces,
                },
                until * time_waiting - (until - self.time - time_waiting) * produces.geode,
            )
        })
    }
}

impl Blueprint {
    fn solve(self, until: u16) -> u16 {
        // we can only build 1 robot each minute. So we don't need more production than we can use in a minute
        let max_ore_robots = u16::max(self.clay.ore, u16::max(self.obsidian.ore, self.geode.ore));
        let max_clay_robots = self.obsidian.clay;
        let max_obsidian_robots = self.geode.obsidian;

        let res = astar(
            &State {
                time: 0,
                store: Default::default(),
                production: Vector {
                    ore: 1,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
            },
            |&s| {
                // should we build an obsidian robot
                let geode_robot = s.build(
                    self.geode,
                    Vector {
                        geode: 1,
                        ..<_>::default()
                    },
                    until,
                );

                // should we build an obsidian robot
                let obsidian_robot = if s.production.obsidian < max_obsidian_robots {
                    // if we can build an obsidian robot at some point, add it to the list
                    s.build(
                        self.obsidian,
                        Vector {
                            obsidian: 1,
                            ..<_>::default()
                        },
                        until,
                    )
                } else {
                    None
                };

                // should we build a clay robot
                let clay_robot = if s.production.clay < max_clay_robots {
                    // if we can build a clay robot at some point, add it to the list
                    s.build(
                        self.clay,
                        Vector {
                            clay: 1,
                            ..<_>::default()
                        },
                        until,
                    )
                } else {
                    None
                };

                // should we build an ore robot
                let ore_robot = if s.production.ore < max_ore_robots {
                    // if we can build an ore robot at some point, add it to the list
                    s.build(
                        self.ore,
                        Vector {
                            ore: 1,
                            ..<_>::default()
                        },
                        until,
                    )
                } else {
                    None
                };

                // do nothing forever
                let do_nothing = if geode_robot.is_none()
                    && obsidian_robot.is_none()
                    && clay_robot.is_none()
                    && ore_robot.is_none()
                {
                    let time_waiting = until - s.time;
                    Some((
                        State {
                            time: until,
                            store: s.store + s.production * time_waiting,
                            production: s.production,
                        },
                        until * time_waiting,
                    ))
                } else {
                    None
                };

                geode_robot
                    .into_iter()
                    .chain(obsidian_robot)
                    .chain(clay_robot)
                    .chain(ore_robot)
                    .chain(do_nothing)
            },
            |s| {
                let time_waiting = until - s.time;
                let goedes = time_waiting * (time_waiting + 1) / 2;
                until * time_waiting - goedes
            },
            |s| s.time == until,
        )
        .unwrap();

        until * until - res.1
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = u16;
    fn part_one(self) -> Self::Output1 {
        let mut sum = 0;
        for (i, bp) in self.0.into_iter().enumerate() {
            sum += (i as u16 + 1) * bp.solve(24)
        }
        sum
    }

    type Output2 = u16;
    fn part_two(self) -> Self::Output2 {
        let mut prod = 1;
        for bp in self.0.into_iter().take(3) {
            prod *= bp.solve(32)
        }
        prod
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
        assert_eq!(output.part_two(), 3472);
    }
}
