#![feature(get_many_mut)]
use aoc::{Challenge, Parser as ChallengeParser};
use arrayvec::ArrayVec;
use nom::IResult;
use pathfinding::directed::astar;

#[derive(Debug, PartialEq, Clone)]
pub struct Valve {
    name: &'static str,
    flow_rate: i32,
    // first value is the index of the valve, second value is the time it takes to get there
    leads_to: ArrayVec<(usize, usize), 64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(usize, ArrayVec<Valve, 64>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let mut x = ArrayVec::<&'static str, 64>::new();
        let mut y = ArrayVec::<Valve, 64>::new();

        let mut start = 0;
        for (i, line) in input.lines().enumerate() {
            let name = &line[6..8];
            if name == "AA" {
                start = i;
            }
            x.push(name);
        }

        for (i, line) in input.lines().enumerate() {
            let Some((_, line)) = line.split_once('=') else { continue; };
            let Some((flow_rate, line)) = line.split_once("; ") else { continue; };
            let flow_rate = flow_rate.parse().unwrap();

            let mut leads_to = ArrayVec::new();
            if let Some(singular) = line.strip_prefix("tunnel leads to valve ") {
                let idx = x.iter().position(|x| *x == singular).unwrap();
                leads_to.push((idx, 1));
            } else if let Some(many) = line.strip_prefix("tunnels lead to valves ") {
                for leads in many.split(", ") {
                    let idx = x.iter().position(|x| *x == leads).unwrap();
                    leads_to.push((idx, 1));
                }
            }

            y.push(Valve {
                name: x[i],
                flow_rate,
                leads_to,
            });
        }

        #[allow(clippy::never_loop)]
        for i in 0..y.len() {
            let mut dont_track = ArrayVec::<usize, 64>::new();
            dont_track.push(i);
            let mut j = 0;
            while j < y[i].leads_to.len() {
                let (k, t) = y[i].leads_to[j];
                let Ok([first, second]) = y.get_many_mut([i, k]) else {
                    j += 1;
                    continue;
                };

                if second.flow_rate > 0 {
                    j += 1;
                    continue;
                }

                // if this lead is useless since the flow_rate is zero.
                // remove it and copy it's leads in
                first.leads_to.remove(j);
                dont_track.push(k);

                // follow the leads from k
                for (m, t1) in second.leads_to.iter().copied() {
                    if dont_track.contains(&m) {
                        continue;
                    }
                    let p = first.leads_to.iter().position(|x| x.0 == m);
                    // only push if we don't have this path already
                    // if we do have this path, update it
                    if let Some(p) = p {
                        first.leads_to[p].1 = usize::min(first.leads_to[p].1, t + t1);
                    } else {
                        first.leads_to.push((m, t + t1));
                    }
                }
            }
        }

        Ok(("", Self(start, y)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    type Output1 = usize;
    fn part_one(self) -> Self::Output1 {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        struct Position {
            valve: usize,
            // bit 1 means valve at that index is open
            state: u64,
            time: usize,
        }

        let max = 30 * self.1.iter().map(|x| x.flow_rate).max().unwrap_or(0);

        let output = astar::astar(
            &Position {
                valve: self.0,
                state: 0,
                time: 1,
            },
            |p| {
                let Position {
                    valve,
                    state,
                    time,
                    // rate,
                } = *p;
                let Valve {
                    flow_rate,
                    ref leads_to,
                    ..
                } = self.1[valve];

                let valve_open = state | (1 << valve);
                let rate = if state == valve_open {
                    0
                } else {
                    flow_rate * (30 - time as i32)
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
                }

                impl Iterator for Iter {
                    type Item = (Position, i32);
                    fn next(&mut self) -> Option<Self::Item> {
                        loop {
                            let (lead, t) = self.leads_to.next()?;
                            if self.time + t > 30 {
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
                    // rate,
                    time,
                    state,
                    max,
                })
            },
            |p| {
                let time_remaining = 30 - p.time as i32;
                let mut flow_remaining = max;
                for (i, valve) in self.1.iter().enumerate() {
                    if (p.state >> i) & 1 == 0 {
                        flow_remaining -= valve.flow_rate;
                    }
                }
                time_remaining * flow_remaining
            },
            |p| p.time == 30,
        )
        .unwrap();

        (29 * max - output.1) as usize
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
        assert_eq!(output.part_two(), 0);
    }
}
