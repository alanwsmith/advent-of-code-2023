use day_06_part_01::Solver;

fn main() {
    divan::main();
}

#[divan::bench]
fn solve_test() {
    let mut s = Solver::new();
    s.input = Some(include_str!("../input.txt").to_string());
    s.solve();
}