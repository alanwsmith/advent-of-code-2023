use day_07_part_01_version_01::Solver;

fn main() {
    divan::main();
}

#[divan::bench]
fn solve_test() {
    let mut s = Solver::new();
    s.input = Some(include_str!("../input-test.txt").to_string());
    s.solve();
}
