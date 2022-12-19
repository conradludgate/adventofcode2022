#![feature(get_many_mut)]
use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::IResult;
use pathfinding::directed::astar;

#[derive(Debug, PartialEq, Clone)]
pub struct ValveInit {
    name: &'static str,
    flow_rate: usize,
    // first value is the index of the valve, second value is the time it takes to get there
    leads_to: Vec<&'static str>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Valve {
    name: &'static str,
    flow_rate: usize,
    // first value is the index of the valve, second value is the time it takes to get there
    leads_to: ArrayVec<(usize, usize), 64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    start: usize,
    valves: ArrayVec<Valve, 64>,
}

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
        let mut valves = ArrayVec::<Valve, 64>::new();

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

            valves.push(Valve {
                name: valve.name,
                flow_rate: valve.flow_rate,
                leads_to,
            });
        }

        // Floyd Warshall
        let v = valves.len();
        for k in 1..v {
            for i in 0..v {
                for j in 0..v {
                    if i == j {
                        continue;
                    }
                    let Some(&(_, b)) = valves[i].leads_to.iter().find(|x| x.0 == k) else { continue };
                    let Some(&(_, c)) = valves[k].leads_to.iter().find(|x| x.0 == j) else { continue };
                    let d = b.saturating_add(c);
                    if let Some(idx) = valves[i].leads_to.iter().position(|x| x.0 == j) {
                        if valves[i].leads_to[idx].1 > d {
                            valves[i].leads_to[idx].1 = d;
                        }
                    } else {
                        valves[i].leads_to.push((j, d))
                    }
                }
            }
        }
        for i in 0..v {
            valves[i].leads_to.retain(|x| init[x.0].flow_rate > 0)
        }

        Ok(("", Self { start, valves }))
    }
}

impl Solution {
    fn solve(&self, steps: usize, until: usize) -> usize {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        struct Position {
            valve: usize,
            // bit 1 means valve at that index is open
            state: u64,
            time: usize,
        }

        // a max possible flow reduction per step
        // this is needed as we want to find the 'shortest path', where shortest means the most pressure released.
        // we model this as `-flow_rate * time_remaining`, but A* doesn't like negative weighted edges,
        // so we must use `max - flow_rate * time_remaining` instead.
        let max = steps * self.valves.iter().map(|x| x.flow_rate).max().unwrap_or(0);

        let res = astar::astar(
            &Position {
                valve: self.start,
                state: 0,
                time: 0,
            },
            |&Position { valve, state, time }| {
                let iter = self.valves[valve]
                    .leads_to
                    .clone()
                    .into_iter()
                    // filter out leads that don't have enough time to process
                    .filter(move |(_, t)| {
                        // new time will be time it takes to travel, + 1 minute to open the valve
                        let new_t = time + t + 1;

                        (time < steps && new_t < steps) || (time >= steps && new_t < steps * 2)
                    })
                    // filter out leads that already have the valve open
                    .filter(move |(lead, _)| (state >> lead) & 1 == 0)
                    .map(move |(lead, t)| {
                        let scale = (t + 1) * max;
                        let flow = self.valves[lead].flow_rate;

                        let new_t = time + t + 1;
                        let time_remaining = steps - 1 - (new_t % steps);
                        (
                            Position {
                                valve: lead,
                                time: new_t,
                                state: state | 1 << lead,
                            },
                            scale - flow * time_remaining,
                        )
                    });

                // support edge case where there's no possible moves to make in time
                let idle = if time < steps && steps < until {
                    // if we have the elephant path afterwards, fast forward to that
                    (
                        Position {
                            valve: self.start,
                            time: steps,
                            state,
                        },
                        (steps - time - 1) * max,
                    )
                } else {
                    // fast forward to finished time
                    (
                        Position {
                            valve,
                            time: until - 1,
                            state,
                        },
                        (until - time - 1) * max,
                    )
                };
                iter.chain(std::iter::once(idle))
            },
            |p| {
                // assuming that our self.1 is sorted from most to least flow
                // and for the heuristic, assume that we can make it to each valve in 2 time step
                let mut time_remaining = until - p.time;
                let mut flow_remaining = max * time_remaining;
                for (i, valve) in self.valves.iter().enumerate() {
                    if (p.state >> i) & 1 == 0 {
                        flow_remaining -= valve.flow_rate * time_remaining;
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
        .unwrap();

        // this should be roughly `(max * steps) - res.1` but that's giving me different answers for some reason...
        let mut total = 0;
        for i in &res.0[..res.0.len() - 1] {
            let flow = self.valves[i.valve].flow_rate;
            if i.time < steps {
                total += flow * (steps - i.time);
            } else {
                total += flow * (until - i.time);
            }
        }
        total
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
