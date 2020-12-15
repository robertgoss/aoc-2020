use num_bigint::BigInt;
use ring_algorithm::chinese_remainder_theorem;

fn solve_remainder_problem(problem : &[(u64, usize)]) -> BigInt {
    let remainders : Vec<BigInt> = problem.iter().map(|(_,r)| BigInt::from(*r)).collect();
    let modulos : Vec<BigInt> = problem.iter().map(|(m,_)| BigInt::from(*m) ).collect();
    -chinese_remainder_theorem::<BigInt>(&remainders, &modulos).unwrap()
}

#[derive(Copy, Clone)]
pub struct Bus {
    period : Option<u64>
}

impl Bus {
    pub fn from_string(string : &str) -> Bus {
        Bus {
            period : string.parse::<u64>().ok()
        }
    }
    pub fn id(self : &Self) -> Option<u64> {
        self.period
    }
    pub fn departs(self : &Self, timestamp : u64) -> bool {
        self.period.map(
            |time| timestamp % time == 0
        ).unwrap_or(false)
    }
}

pub struct Timetable {
    timestamp : u64,
    buses : Vec<Bus>
}
impl Timetable {
    pub fn from_lines(line1 : &str, line2 : &str) -> Timetable {
        Timetable {
            timestamp : line1.parse::<u64>().unwrap(),
            buses : line2.split(",").map(
                |string| Bus::from_string(string)
            ).collect()
        }
    }
    pub fn first_bus(self : &Self) -> Option<(u64, Bus)> {
        (0..).filter_map(
            |index| self.has_bus(index + self.timestamp).map(
                |bus| (index, bus)
            )
        ).next()
    }
    fn has_bus(self : &Self, timestamp : u64) -> Option<Bus> {
        self.buses.iter().filter(
            |bus| bus.departs(timestamp)
        ).next().map(
            |bus| *bus
        )
    }
    pub fn first_congunction(self : &Self) -> BigInt {
        let remainder_problem : Vec<(u64, usize)> =
            self.buses.iter().enumerate().filter_map(
                |(i,bus)| bus.id().map(|modulo| (modulo, i))
            ).collect();
        solve_remainder_problem(&remainder_problem)
    }
}