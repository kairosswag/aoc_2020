use parse_display::{Display, FromStr};

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct BusSchedule {
    pub earliest_start: u32,
    pub buses: Vec<Departure>,
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy, Eq)]
pub enum Departure {
    #[display("{0}")]
    Bus(u32),
    #[display("x")]
    X,
}

impl Departure {
    pub fn get_bus_id(&self) -> Option<u32> {
        match self {
            Departure::Bus(val) => Some(*val),
            Departure::X => None,
        }
    }
}

#[aoc_generator(day13)]
pub fn generate(input: &str) -> BusSchedule {
    let mut lines = input.lines();
    let earliest_start = lines
        .next()
        .expect("first line")
        .parse::<u32>()
        .expect("first line no number");
    let buses = lines
        .next()
        .map(|s| {
            s.split(',')
                .map(|s| {
                    s.parse::<Departure>()
                        .expect("departure could not be found")
                })
                .collect()
        })
        .expect("could not parse line");

    BusSchedule {
        earliest_start,
        buses,
    }
}

#[aoc(day13, part1)]
pub fn part1(sched: &BusSchedule) -> u32 {
    let (bus_id, delta) = sched
        .buses
        .iter()
        .flat_map(|b| b.get_bus_id())
        .map(|t| (t, get_next_higher_multiple(sched.earliest_start, t)))
        .map(|(bus_id, next)| (bus_id, next - sched.earliest_start))
        .fold(
            (0, std::u32::MAX),
            |(acc_bus_id, acc_min), (bus_id, delta)| {
                if delta < acc_min {
                    (bus_id, delta)
                } else {
                    (acc_bus_id, acc_min)
                }
            },
        );
    bus_id * delta
}

pub fn get_next_higher_multiple(earliest: u32, period: u32) -> u32 {
    let multi = earliest / period;
    if multi * period == earliest {
        multi * period
    } else {
        (multi + 1) * period
    }
}

#[aoc(day13, part2)]
pub fn part2(sched: &BusSchedule) -> u64 {
    let buses = &sched.buses;
    let mut bus_offs: Vec<(u64, u64)> = Vec::new();
    for i in 0..buses.len() {
        if let Some(id) = buses[i].get_bus_id() {
            bus_offs.push((id as u64, i as u64));
        }
    }

    let mut idx = 0;
    let mut step = 1;
    let mut bus_iter = bus_offs.iter();
    let (mut bus, mut dlt) = bus_iter.next().expect("wut");
    loop {
        if (idx + dlt) % bus == 0 {
            // println!("found next step at idx: {}", idx);
            step *= bus;
            // println!("step is set to: {}", step);
            let curr_search = bus_iter.next();
            if let Some((n_bus, n_delta)) = curr_search {
                // println!("Continue with bus {} at delta {}", n_bus, n_delta);
                bus = *n_bus;
                dlt = *n_delta;
            } else {
                return idx;
            }
        } else {
            idx += step;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test() {
        let test = "939
7,13,x,x,59,x,31,19
        ";

        let gen = generate(&test);
        assert_eq!(1068781, part2(&gen));
    }
}
