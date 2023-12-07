use day_05_part_02_v2::Solver;

fn main() {
    divan::main();
}

#[divan::bench]
fn solve_test() {
    let mut s = Solver::new();
    s.input = Some(include_str!("../input-test.txt").to_string());
    s.solve();
}
