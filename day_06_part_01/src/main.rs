use day_06_part_01::Race;
use day_06_part_01::Solver;

fn main() {
    let mut s = Solver::new();
    s.races = vec![
        Race { time: 45, record: 305 },
        Race { time: 97, record: 1062 },
        Race { time: 72, record: 1110 },
        Race { time: 95, record: 1696 },
    ];
    s.input = Some(include_str!("../input.txt").to_string());
    dbg!(s.solve());
}
