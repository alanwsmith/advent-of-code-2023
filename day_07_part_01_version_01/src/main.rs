use day_07_part_01_version_01::Solver;

fn main() {
    let mut s = Solver::new();
    s.input = Some(include_str!("../input.txt").to_string());
    dbg!(s.solve());
}
