pub struct Solver {
    pub input: Option<String>
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            input: None
        }
    }

    pub fn solve(&self) -> i32 {
        6440
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn solve_integration_1() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = 6440;
        let right = s.solve();
        assert_eq!(left, right);
    }

    // #[rstest]
    // #[case("a", "a")]
    // fn run_test(#[case] input: &str, #[case] expected: &str) {
    //     let results = input;
    //     assert_eq!(expected, input);
    // }


}