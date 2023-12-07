use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
struct Card {
    line: Option<String>,
    cache_picks: Vec<usize>,
    cache_winners: Vec<usize>,
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
        Card {
            line: None,
            cache_picks: vec![],
            cache_winners: vec![],
        }
    }

    fn picks(&self) -> Vec<usize> {
        if self.cache_picks.len() > 0 {
            self.cache_picks.clone()
        } else {
            self.get_picks().unwrap().1
        }
    }

    fn winners(&self) -> Vec<usize> {
        if self.cache_winners.len() > 0 {
            self.cache_winners.clone()
        } else {
            self.get_winners().unwrap().1
        }
    }

    fn winner_count(&self) -> usize {
        self.winners()
            .into_iter()
            .filter(|winner| self.picks().contains(winner))
            .fold(0, |acc, _| acc + 1)
    }
}

pub struct Solver {
    pub input: Option<String>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver { input: None }
    }

    pub fn solve(&mut self) -> usize {
        let input = self.input.clone().unwrap();
        let mut updates: Vec<usize> = vec![0];
        let response = input.lines().fold(0, |acc, line| {
            // dbg!(&updates);
            let mut c = Card::new();
            c.line = Some(line.to_string());
            for _ in 0..=updates[0] {
                for num in 0..c.winner_count() {
                    if num > updates.len() - 1 {
                        updates.push(0)
                    }
                    updates[num] += 1;
                }
            }
            let new_value = acc + updates[0] + 1;
            for u in 0..updates.len() - 1 {
                updates[u] = updates[u + 1];
                if u == updates.len() - 1 {
                    updates[u] = 0;
                }
            }
            new_value
        });
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn winner_count_test() {
        let mut c = Card::new();
        c.line = Some("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string());
        let left = 4;
        let right = c.winner_count();
        assert_eq!(left, right);
    }

    #[test]
    fn solve_count_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let mut s = Solver::new();
        s.input = Some(input.to_string());
        let left = 30;
        let right = s.solve();
        assert_eq!(left, right);
    }

    #[test]
    fn solve_count_test_2() {
        let input = "Card 1: 1 2 | 3 4";
        let mut s = Solver::new();
        s.input = Some(input.to_string());
        let left = 1;
        let right = s.solve();
        assert_eq!(left, right);
    }
}
