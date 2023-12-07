use day_06_part_02::Race;
use day_06_part_02::Solver;

fn main() {
    divan::main();
}

#[divan::bench]
fn solve_test() {
    let mut s = Solver::new();
    s.races = vec![
        Race { time: 45977295, record: 305106211101695 },
    ];
    s.solve();
}
