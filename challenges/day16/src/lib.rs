#![feature(get_many_mut)]
use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::IResult;
use pathfinding::directed::astar;

#[derive(Debug, PartialEq, Clone)]
pub struct ValveInit {
    name: &'static str,
    flow_rate: i32,
    // first value is the index of the valve, second value is the time it takes to get there
    leads_to: Vec<&'static str>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Valve {
    flow_rate: i32,
    // first value is the index of the valve, second value is the time it takes to get there
    leads_to: ArrayVec<(usize, usize), 64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(usize, ArrayVec<Valve, 64>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut init = ArrayVec::<ValveInit, 64>::new();

        for line in input.lines() {
            let name = &line[6..8];

            let Some((_, line)) = line.split_once('=') else { continue; };
            let Some((flow_rate, line)) = line.split_once("; ") else { continue; };
            let flow_rate = flow_rate.parse().unwrap();

            let leads_to = if let Some(singular) = line.strip_prefix("tunnel leads to valve ") {
                vec![singular]
            } else if let Some(many) = line.strip_prefix("tunnels lead to valves ") {
                many.split(", ").collect()
            } else {
                vec![]
            };

            init.push(ValveInit {
                name,
                flow_rate,
                leads_to,
            });
        }

        // we want highest flow at the beginning - this helps our heuristics
        init.sort_unstable_by_key(|x| std::cmp::Reverse(x.flow_rate));

        let mut start = 0;
        let mut y = ArrayVec::<Valve, 64>::new();

        for (i, valve) in init.iter().enumerate() {
            let leads_to = valve
                .leads_to
                .iter()
                .flat_map(|&s| init.iter().position(|v| v.name == s))
                .map(|j| (j, 1))
                .collect();

            if valve.name == "AA" {
                start = i;
            }

            y.push(Valve {
                flow_rate: valve.flow_rate,
                leads_to,
            });
        }

        // Floyd Warshall
        let v = y.len();
        for k in 1..v {
            for i in 0..v {
                for j in 0..v {
                    if i == j {
                        continue;
                    }
                    let Some(&(_, b)) = y[i].leads_to.iter().find(|x| x.0 == k) else { continue };
                    let Some(&(_, c)) = y[k].leads_to.iter().find(|x| x.0 == j) else { continue };
                    let d = b.saturating_add(c);
                    if let Some(idx) = y[i].leads_to.iter().position(|x| x.0 == j) {
                        if y[i].leads_to[idx].1 > d {
                            y[i].leads_to[idx].1 = d;
                        }
                    } else {
                        y[i].leads_to.push((j, d))
                    }
                }
            }
        }
        for i in 0..v {
            y[i].leads_to.retain(|x| init[x.0].flow_rate > 0)
        }

        Ok(("", Self(start, y)))
    }
}

impl Solution {
    fn solve(&self, steps: i32, until: usize) -> usize {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        struct Position {
            valve: usize,
            // bit 1 means valve at that index is open
            state: u64,
            time: usize,
        }

        let max = steps * self.1.iter().map(|x| x.flow_rate).max().unwrap_or(0);
        let mut count = 0;

        let dist = astar::astar(
            &Position {
                valve: self.0,
                state: 0,
                time: 0,
            },
            |p| {
                count += 1;
                let Position { valve, state, time } = *p;
                let Valve {
                    flow_rate,
                    ref leads_to,
                } = self.1[valve];

                let valve_open = state | (1 << valve);
                let rate = if state == valve_open {
                    0
                } else {
                    flow_rate * (steps - 1 - (time as i32) % steps)
                };

                let identity = std::iter::once((
                    Position {
                        valve: p.valve,
                        state: valve_open,
                        time: time + 1,
                    },
                    max - rate,
                ));

                struct Iter {
                    leads_to: arrayvec::IntoIter<(usize, usize), 64>,
                    time: usize,
                    state: u64,
                    max: i32,
                    steps: usize,
                }

                impl Iterator for Iter {
                    type Item = (Position, i32);
                    fn next(&mut self) -> Option<Self::Item> {
                        loop {
                            let (lead, t) = self.leads_to.next()?;
                            if (self.state >> lead) & 1 == 1 {
                                continue;
                            }

                            if (self.time < self.steps && self.time + t >= self.steps)
                                || (self.time >= self.steps && self.time + t >= self.steps * 2)
                            {
                                continue;
                            }

                            break Some((
                                Position {
                                    valve: lead,
                                    time: self.time + t,
                                    state: self.state,
                                },
                                self.max * t as i32,
                            ));
                        }
                    }
                }

                identity.chain(Iter {
                    leads_to: leads_to.clone().into_iter(),
                    time,
                    state,
                    max,
                    steps: steps as usize,
                })
            },
            |p| {
                // assuming that our self.1 is sorted from most to least flow
                // and for the heuristic, assume that we can make it to each valve in 2 time step
                let mut time_remaining = until - p.time;
                let mut flow_remaining = max * time_remaining as i32;
                for (i, valve) in self.1.iter().enumerate() {
                    if (p.state >> i) & 1 == 0 {
                        flow_remaining -= valve.flow_rate * time_remaining as i32;
                        if time_remaining < 2 {
                            break;
                        }
                        time_remaining -= 2;
                    }
                }
                flow_remaining
            },
            |p| p.time + 1 == until,
        )
        .unwrap()
        .1;

        // dbg!(count);
        (until - 1) * (max as usize) - (dist as usize)
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        self.solve(30, 30)
    }

    type Output2 = usize;
    fn part_two(self) -> Self::Output2 {
        self.solve(26, 52)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:#?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one(), 1651);
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two(), 1707);
    }
}
