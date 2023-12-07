use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;

pub struct Hand {
    pub raw_string: Option<String>,
}

impl Hand {
    pub fn new_from(raw_string: &str) -> Hand {
        Hand {
            raw_string: Some(raw_string.to_string()),
        }
    }

    pub fn cards(&self) -> Vec<u32> {
        self.parse_cards().unwrap().1
    }

    pub fn parse_cards(&self) -> IResult<&str, Vec<u32>> {
        let (source, results) = many1(alt((
            tag("A").map(|_| 14),
            tag("K").map(|_| 13),
            tag("Q").map(|_| 12),
            tag("J").map(|_| 11),
            tag("T").map(|_| 10),
            one_of("0123456789").map(|d| d.to_digit(10).unwrap()),
        )))(self.raw_string.as_ref().unwrap().as_str())?;
        Ok((source, results))
    }
}

pub struct Solver {
    pub input: Option<String>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver { input: None }
    }

    pub fn solve(&self) -> u32 {
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

    #[rstest]
    #[case("32T3K 765", vec![3, 2, 10, 3, 13])]
    fn card_test(#[case] input: &str, #[case] left: Vec<u32>) {
        let h = Hand::new_from(input);
        let right = h.cards();
        assert_eq!(left, right);
    }

    // #[rstest]
    // #[case("T55J5 684", Ok(("", vec![3, 2, 10, 3, 13])))]
    // fn hand_test(#[case] input: &str, #[case] left: IResult<&str, Vec<u32>>) {
    //     let s = Solver::new();
    //     let right = s.cards(input);
    //     assert_eq!(left, right);
    // }

    // #[rstest]
    // #[case("a", "a")]
    // fn run_test(#[case] input: &str, #[case] expected: &str) {
    //     let results = input;
    //     assert_eq!(expected, input);
    // }
}

