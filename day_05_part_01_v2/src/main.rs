use day_05_part_01_v2::Solver;

fn main() {
    let mut s = Solver::new();
    s.input = Some(include_str!("../input.txt").to_string());
    dbg!(s.solve());
}
