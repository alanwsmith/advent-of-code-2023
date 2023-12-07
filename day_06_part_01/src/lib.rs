pub struct Race {
    pub time: u32,
    pub record: u32,
}

impl Race {
    pub fn possible_solutions(&self) -> u32 {
        let mut new_records = 0;
        for ms in 0..=self.time {
            let speed = ms;
            let tics = self.time - ms;
            let distance = speed * tics;
            if distance > self.record {
                new_records += 1;
            }
        }
        new_records
    }
}

pub struct Solver {
    pub input: Option<String>,
    pub races: Vec<Race>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            input: None,
            races: vec![],
        }
    }

    pub fn solve(&self) -> u32 {
        self.races
            .iter()
            .fold(1, |acc, race| acc * race.possible_solutions())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_1() {
        let mut s = Solver::new();
        s.races = vec![
            Race { time: 7, record: 9 },
            Race {
                time: 15,
                record: 40,
            },
            Race {
                time: 30,
                record: 200,
            },
        ];
        let left = 288;
        let right = s.solve();
        assert_eq!(left, right);
    }

    #[test]
    fn race_possibilities() {
        let r = Race { time: 7, record: 9 };
        let left = 4;
        let right = r.possible_solutions();
        assert_eq!(left, right);
    }
}
