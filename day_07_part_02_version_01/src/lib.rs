use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::digit1;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub struct Hand {
    pub raw_string: Option<String>,
    pub rank: u128,
}

impl Hand {
    pub fn new_from(raw_string: &str) -> Hand {
        Hand {
            raw_string: Some(raw_string.to_string()),
            rank: 0,
        }
    }

    pub fn cards(&self) -> Vec<u32> {
        self.parse_cards().unwrap().1
    }

    pub fn hand_strength(&self) -> u128 {
        let cards = self.cards();
        let mut strength = 0;
        strength += cards[4] as u128;
        strength += (cards[3] * 100) as u128;
        strength += (cards[2] * 10000) as u128;
        strength += (cards[1] * 1000000) as u128;
        strength += (cards[0] * 100000000) as u128;
        strength += self.kind_strength();
        strength
    }

    pub fn joker_count(&self) -> u128 {
        self.cards()
            .iter()
            .filter(|c| if c == &&1 { true } else { false })
            .count()
            .try_into()
            .unwrap()
    }

    pub fn kind_strength(&self) -> u128 {
        let mut counts = vec![0 as u128; 15];
        self.cards()
            .iter()
            .filter(|c| if c > &&1 { true } else { false })
            .for_each(|c| counts[*c as usize] += 1);
        let mut report: Vec<_> = counts
            .into_iter()
            .filter(|e| if e > &&1 { true } else { false })
            .collect();
        report.sort();
        report.reverse();
        if report.len() > 0 {
            report[0] += self.joker_count()
        } else if self.joker_count() > 0 {
            if self.joker_count() == 5 {
                report.push(5)
            } else {
                report.push(self.joker_count() + 1)
            }
        }

        dbg!(&report);
        if report == vec![5] {
            70000000000
        } else if report == vec![4] {
            60000000000
        } else if report == vec![3, 2] {
            50000000000
        } else if report == vec![3] {
            40000000000
        } else if report == vec![2, 2] {
            30000000000
        } else if report == vec![2] {
            20000000000
        } else {
            10000000000
        }
    }

    pub fn parse_cards(&self) -> IResult<&str, Vec<u32>> {
        let (source, results) = many1(alt((
            tag("A").map(|_| 14),
            tag("K").map(|_| 13),
            tag("Q").map(|_| 12),
            tag("J").map(|_| 1),
            tag("T").map(|_| 10),
            one_of("0123456789").map(|d| d.to_digit(10).unwrap()),
        )))(self.raw_string.as_ref().unwrap().as_str())?;
        Ok((source, results))
    }

    pub fn parse_bid(&self) -> IResult<&str, u128> {
        let (source, results) = preceded(pair(take_until(" "), space1), digit1)(
            self.raw_string.as_ref().unwrap().as_str(),
        )?;
        Ok((source, results.parse().unwrap()))
    }

    pub fn points(&self) -> u128 {
        self.rank * self.parse_bid().unwrap().1
    }
}

pub struct Solver {
    pub input: Option<String>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver { input: None }
    }

    pub fn hands(&self) -> Vec<Hand> {
        let mut hands: Vec<Hand> = self
            .input
            .as_ref()
            .unwrap()
            .lines()
            .map(|l| {
                let h = Hand::new_from(l);
                h
            })
            .collect();
        hands.sort_by(|a, b| a.hand_strength().cmp(&b.hand_strength()));
        for (rank, hand) in hands.iter_mut().enumerate() {
            hand.rank = (rank + 1) as u128;
        }
        hands
    }

    pub fn solve(&self) -> u128 {
        self.hands().iter().fold(0, |acc, hand| acc + hand.points())
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
        let left = 5905;
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

    #[rstest]
    #[case(include_str!("../input-test.txt"), 5)]
    fn hands_test(#[case] input: &str, #[case] left: u32) {
        let mut s = Solver::new();
        s.input = Some(input.to_string());
        let right = s.hands().len() as u32;
        assert_eq!(left, right);
    }

    #[rstest]
    #[case("23456 1", 10000000000)] // high card
    #[case("33456 1", 20000000000)] // one pair
    #[case("33556 1", 30000000000)] // two pair
    #[case("33356 1", 40000000000)] // three of a kind
    #[case("33355 1", 50000000000)] // full house
    #[case("33335 1", 60000000000)] // four of a kind
    #[case("33333 1", 70000000000)] // five of a kind
    #[case("J3456 1", 20000000000)] // joker one pair
    #[case("JJ456 1", 40000000000)] // joker three of a kind
    #[case("JJJ56 1", 60000000000)] // joker four of a kind
    #[case("JJJJ6 1", 70000000000)] // joker five of a kind
    #[case("JJJJJ 1", 70000000000)] // all jokers
    #[case("J4455 1", 50000000000)] // joker full house
    #[case("T55J5 1", 60000000000)] // joker four of a kind
    #[case("22255 1", 50000000000)] // full house
    #[case("22J55 1", 50000000000)] // full house
    fn kind_strength_test(#[case] input: &str, #[case] left: u128) {
        let h = Hand::new_from(input);
        let right = h.kind_strength();
        assert_eq!(left, right);
    }

    #[rstest]
    #[case("T55J5 1", 1)]
    #[case("23456 1", 0)]
    fn joker_count_test(#[case] input: &str, #[case] left: u128) {
        let h = Hand::new_from(input);
        let right = h.joker_count();
        assert_eq!(left, right);
    }

    #[rstest]
    #[case("12345 1", 10102030405)]
    fn hand_strength_test(#[case] input: &str, #[case] left: u128) {
        let h = Hand::new_from(input);
        let right = h.hand_strength();
        assert_eq!(left, right);
    }

    #[rstest]
    #[case("QQQJA 483", 5, 2415)]
    fn hand_points_test(#[case] input: &str, #[case] rank: u128, #[case] left: u128) {
        let mut h = Hand::new_from(input);
        h.rank = rank;
        let right = h.points();
        assert_eq!(left, right);
    }
}
