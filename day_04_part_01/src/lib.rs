#![allow(dead_code)]
#![allow(unused_imports)]

use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug)]
struct Card {
    line: Option<String>,
}

impl Card {
    fn get_picks(&self) -> IResult<&str, Vec<usize>> {
        let source = self.line.as_ref().unwrap();
        let (source, _) = take_until("|")(source.as_str())?;
        let (source, _) = pair(tag("|"), space1)(source)?;
        let (_, pick_strings) = separated_list1(space1, digit1)(source)?;
        let picks: Vec<usize> = pick_strings
            .into_iter()
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        Ok(("", picks))
    }

    fn get_winners(&self) -> IResult<&str, Vec<usize>> {
        let source = self.line.as_ref().unwrap();
        let (source, _) = tuple((take_until(":"), tag(":"), space1))(source.as_str())?;
        let (_, results) = take_until("|")(source)?;
        let (_, winner_strings) = separated_list1(space1, digit1)(results)?;
        let winners: Vec<usize> = winner_strings
            .into_iter()
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        Ok(("", winners))
    }

    fn new() -> Card {
        Card { line: None }
    }

    fn points(&self) -> usize {
        let mut counter = 0b1;
        self.winners().iter().for_each(|winner| {
            if self.picks().contains(winner) {
                counter = counter << 1;
            }
            ()
        });
        counter >> 1
    }

    fn picks(&self) -> Vec<usize> {
        self.get_picks().unwrap().1
    }

    fn winners(&self) -> Vec<usize> {
        self.get_winners().unwrap().1
    }
}

struct Solver {
    input: Option<String>,
    cards: Vec<Card>,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            input: None,
            cards: vec![],
        }
    }

    fn solve(&mut self) -> usize {
        let input = self.input.clone().unwrap();
        input.lines().for_each(|line| {
            let mut c = Card::new();
            c.line = Some(line.to_string());
            self.cards.push(c);
        });
        let total_points = self.cards.iter().fold(0, |acc, card| acc + card.points());
        total_points
    }
}

fn main() {
    let mut s = Solver::new();
    s.input = Some(include_str!("./input1.txt").to_string());
    dbg!(s.solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_card_test() {
        let mut c = Card::new();
        c.line = Some("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string());
        let left = 2;
        let right = c.points();
        assert_eq!(left, right);
    }

    #[test]
    fn picks_card_test() {
        let mut c = Card::new();
        c.line = Some("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string());
        let left = vec![61, 30, 68, 82, 17, 32, 24, 19];
        let right = c.picks();
        assert_eq!(left, right);
    }

    #[test]
    fn winners_card_test() {
        let mut c = Card::new();
        c.line = Some("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string());
        let left = vec![13, 32, 20, 16, 61];
        let right = c.winners();
        assert_eq!(left, right);
    }

    #[test]
    fn solve_test_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let mut s = Solver::new();
        s.input = Some(input.to_string());
        let left = 13;
        let right = s.solve();
        assert_eq!(left, right);
    }
}
