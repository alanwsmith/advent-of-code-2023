use day_04_part_01::Solver;

fn main() {
    let mut s = Solver::new();
    s.input = Some(include_str!("../input.txt").to_string());
    dbg!(s.solve());
}
