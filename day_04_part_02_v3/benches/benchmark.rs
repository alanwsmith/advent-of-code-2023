use day_04_part_02_v3::Solver;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn solve_test() {
    let mut s = Solver::new();
    s.input = Some(include_str!("../input.txt").to_string());
    s.solve();
}




// // Define a `fibonacci` function and register it for benchmarking.
// #[divan::bench]
// fn fibonacci() -> u64 {
//     fn compute(n: u64) -> u64 {
//         if n <= 1 {
//             1
//         } else {
//             compute(n - 2) + compute(n - 1)
//         }
//     }
//     compute(divan::black_box(10))
// }
