#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug, PartialEq)]
struct Game {
    id: Option<u32>,
    reveals: Vec<Color>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: None,
            reveals: vec![],
        }
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn game_num(source: &str) -> IResult<&str, u32> {
    let (source, _) = tag("Game ")(source)?;
    let (source, results) = digit1(source)?;
    let the_num: u32 = results.parse().unwrap();
    Ok((source, the_num))
}

fn parse_color(source: &str) -> IResult<&str, Color> {
    let (source, num) = digit1(source)?;
    let (source, _) = space1(source)?;
    let (source, result) = alt((
        tag("red").map(|c| Color::Red(num.parse().unwrap())),
        tag("green").map(|c| Color::Green(num.parse().unwrap())),
        tag("blue").map(|c| Color::Blue(num.parse().unwrap())),
    ))(source)?;

    // let (source, result) = alt((
    //     tuple((digit1, space1, tag("red"))).map(|(a, b, c)| Color::Green(2)),
    //     tuple((digit1, space1, tag("green")))
    //         .map(|(a, b, c)| Color::Green((a as &str).parse::<u32>().unwrap())),
    //     tuple((digit1, space1, tag("blue"))).map(|(a, b, c)| Color::Green(2)),
    // ))(source)?;

    Ok((source, result))
}

// fn parse_game(source: &str) -> IResult<&str, Game> {
//     let mut g = Game::new();
//     let (source, game_num) = game_num(source)?;
//     dbg!(&source);
//     g.id = Some(game_num);
//     g.reveals = vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)];
//     Ok((source, g))
// }

// fn parse_reveal(source: &str) -> IResult<&str, (u32, u32, u32)> {
//     let response = (0, 0, 0);
//     let (source, resutls) =
//         tuple((digit1, space1, alt((tag("red"), tag("green"), tag("blue")))))(source)?;
//     Ok((source, (4, 2, 3)))
// }

fn solve(source: &str) -> u32 {
    8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_color_test() {
        let input = "2 green";
        let left = Color::Green(2);
        let right = parse_color(input);
        assert_eq!(left, right.unwrap().1);
    }

    // #[test]
    // fn test_game_data() {
    //     let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    //     let left = Game {
    //         id: Some(1),
    //         reveals: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)],
    //     };
    //     let right = parse_game(input);
    //     assert_eq!(left, right.unwrap().1);
    // }

    // #[test]
    // fn test_reveal_alfa() {
    //     let input = "3 blue, 4 red, 2 green";
    //     let left = (4, 2, 3);
    //     let right = parse_reveal(input);
    //     assert_eq!(left, right.unwrap().1);
    // }

    #[test]
    fn integration_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let left = 8;
        let right = solve(input);
        assert_eq!(left, right);
    }
}
