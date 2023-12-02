#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::multi::separated_list1;
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
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Reveal {
    pub fn new() -> Reveal {
        Reveal {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: Option<u32>,
    reveals: Vec<Vec<Color>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: None,
            reveals: vec![],
        }
    }

    pub fn max_red(&self) -> u32 {
        0
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

// fn load_game(source: &str) -> Game {
//     let g = Game::new();
//     g.id = game_num(source);.unwrap().1;
//     g
// }

fn parse_color(source: &str) -> IResult<&str, Color> {
    let (source, num) = digit1(source)?;
    let (source, _) = space1(source)?;
    let (source, result) = alt((
        tag("red").map(|c| Color::Red(num.parse().unwrap())),
        tag("green").map(|c| Color::Green(num.parse().unwrap())),
        tag("blue").map(|c| Color::Blue(num.parse().unwrap())),
    ))(source)?;
    Ok((source, result))
}

fn parse_colors(source: &str) -> IResult<&str, Vec<Color>> {
    separated_list1(tag(", "), parse_color)(source)
}

fn parse_game_num(source: &str) -> IResult<&str, u32> {
    let (source, results) = tuple((
        tag("Game "),
        digit1.map(|d: &str| d.parse().unwrap()),
        tag(":"),
        space1,
    ))(source)?;
    Ok((source, results.1))
}

// fn parse_game(source: &str) -> IResult<&str, Game> {
//     let g = Game::new();
//     Ok((source, g))
// }

fn parse_reveals(source: &str) -> IResult<&str, Vec<Vec<Color>>> {
    separated_list1(tag("; "), parse_colors)(source)
}

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

    #[test]
    fn parse_colors_test() {
        let input = "4 red, 5 blue";
        let left = vec![Color::Red(4), Color::Blue(5)];
        let right = parse_colors(input);
        assert_eq!(left, right.unwrap().1);
    }

    #[test]
    fn parse_game_num_test() {
        let input = "Game 1: ";
        let left = Ok(("", 1));
        let right = parse_game_num(input);
        assert_eq!(left, right);
    }

    #[test]
    fn parse_reveals_test() {
        let input = "1 blue, 3 red; 4 green";
        let left = vec![vec![Color::Blue(1), Color::Red(3)], vec![Color::Green(4)]];
        let right = parse_reveals(input);
        assert_eq!(left, right.unwrap().1);
    }

    // #[test]
    // fn max_red_test() {
    //     let input = "Game 1: 1 blue, 3 red; 4 green";
    //     let left = 3;
    //     let g = load_game(input);
    //     let right = g.max_red();
    //     assert_eq!(left, right);
    // }

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
