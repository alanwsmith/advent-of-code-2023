use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

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
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug, PartialEq)]
struct Game {
    id: Option<u32>,
    reveals: Vec<Reveal>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: None,
            reveals: vec![],
        }
    }
}

impl Game {
    pub fn max_red(&self) -> u32 {
        self.reveals.iter().map(|e| e.red).max().unwrap()
    }
    pub fn max_green(&self) -> u32 {
        self.reveals.iter().map(|e| e.green).max().unwrap()
    }
    pub fn max_blue(&self) -> u32 {
        self.reveals.iter().map(|e| e.blue).max().unwrap()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn parse_color(source: &str) -> IResult<&str, Color> {
    let (source, num) = digit1(source)?;
    let (source, _) = space1(source)?;
    let (source, result) = alt((
        tag("red").map(|_| Color::Red(num.parse().unwrap())),
        tag("green").map(|_| Color::Green(num.parse().unwrap())),
        tag("blue").map(|_| Color::Blue(num.parse().unwrap())),
    ))(source)?;
    Ok((source, result))
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

fn parse_game(source: &str) -> IResult<&str, Game> {
    let mut g = Game::new();
    let (source, id) = parse_game_num(source)?;
    g.id = Some(id);
    let (source, reveals) = parse_reveals(source)?;
    g.reveals = reveals;
    Ok((source, g))
}

fn parse_games(source: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(tag("\n"), parse_game)(source)
}

fn parse_reveal(source: &str) -> IResult<&str, Reveal> {
    let mut reveal = Reveal::new();
    let (source, colors) = separated_list1(tag(", "), parse_color)(source)?;
    colors.iter().for_each(|color| match color {
        Color::Red(v) => reveal.red = *v,
        Color::Green(v) => reveal.green = *v,
        Color::Blue(v) => reveal.blue = *v,
    });
    Ok((source, reveal))
}

fn parse_reveals(source: &str) -> IResult<&str, Vec<Reveal>> {
    separated_list1(tag("; "), parse_reveal)(source)
}

fn solve(source: &str) -> u32 {
    let games = parse_games(source).unwrap().1;
    games
        .iter()
        .map(|g| g.max_red() * g.max_green() * g.max_blue())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_red_test() {
        let input = "Game 1: 3 red, 2 green; 4 red; 1 blue";
        let g = parse_game(input).unwrap().1;
        let left = 4;
        let right = g.max_red();
        assert_eq!(left, right);
    }

    #[test]
    fn parse_color_test() {
        let input = "2 green";
        let left = Color::Green(2);
        let right = parse_color(input);
        assert_eq!(left, right.unwrap().1);
    }

    #[test]
    fn parse_reveal_test() {
        let input = "4 red, 5 blue";
        let left = Reveal {
            red: 4,
            green: 0,
            blue: 5,
        };
        let right = parse_reveal(input);
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
    fn parse_game_test() {
        let input = "Game 1: 3 red, 2 green; 1 blue";
        let left = Ok((
            "",
            Game {
                id: Some(1),
                reveals: vec![
                    Reveal {
                        red: 3,
                        green: 2,
                        blue: 0,
                    },
                    Reveal {
                        red: 0,
                        green: 0,
                        blue: 1,
                    },
                ],
            },
        ));
        let right = parse_game(input);
        assert_eq!(left, right);
    }

    #[test]
    fn parse_games_test() {
        let input = "Game 1: 3 blue
Game 2: 1 blue";
        let left = Ok((
            "",
            vec![
                Game {
                    id: Some(1),
                    reveals: vec![Reveal {
                        red: 0,
                        green: 0,
                        blue: 3,
                    }],
                },
                Game {
                    id: Some(2),
                    reveals: vec![Reveal {
                        red: 0,
                        green: 0,
                        blue: 1,
                    }],
                },
            ],
        ));
        let right = parse_games(input);
        assert_eq!(left, right);
    }

    #[test]
    fn parse_reveals_test() {
        let input = "1 blue, 3 red; 4 green";
        let left = vec![
            Reveal {
                red: 3,
                green: 0,
                blue: 1,
            },
            Reveal {
                red: 0,
                green: 4,
                blue: 0,
            },
        ];
        let right = parse_reveals(input);
        assert_eq!(left, right.unwrap().1);
    }

    #[test]
    fn part2_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let left = 2286;
        let right = solve(input);
        assert_eq!(left, right);
    }
}
