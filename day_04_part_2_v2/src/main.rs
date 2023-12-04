use day_04_part_2_v2::Solver;

fn main() {
    let mut s = Solver::new();
    s.input = Some(include_str!("../input.txt").to_string());
    dbg!(s.solve());
}
