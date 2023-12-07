use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::separated_list0;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn convert_text(source: &str) -> String {
    let (_, results) = many1(converter)(source).unwrap();
    results.iter().collect()
}

fn converter(source: &str) -> IResult<&str, char> {
    let (source, results) = alt((
        pair(tag("o"), peek(tag("ne"))).map(|_| '1'),
        pair(tag("t"), peek(tag("wo"))).map(|_| '2'),
        pair(tag("t"), peek(tag("hree"))).map(|_| '3'),
        pair(tag("f"), peek(tag("our"))).map(|_| '4'),
        pair(tag("f"), peek(tag("ive"))).map(|_| '5'),
        pair(tag("s"), peek(tag("ix"))).map(|_| '6'),
        pair(tag("s"), peek(tag("even"))).map(|_| '7'),
        pair(tag("e"), peek(tag("ight"))).map(|_| '8'),
        pair(tag("n"), peek(tag("ine"))).map(|_| '9'),
        anychar,
    ))(source)?;
    Ok((source, results))
}

fn solve(source: &str) -> u32 {
    let converted_text = convert_text(source);
    let (_, results) = find_line_digits(&converted_text).unwrap();
    results.iter().fold(0, |acc, e| {
        if e.iter().nth(0).is_some() {
            let text_number = format!("{}{}", e.iter().nth(0).unwrap(), e.iter().last().unwrap());
            acc + text_number.parse::<u32>().unwrap()
        } else {
            acc
        }
    })
}

fn find_line_digits(source: &str) -> IResult<&str, Vec<Vec<char>>> {
    // clear trailing non-digit chars with the opt
    separated_list0(pair(opt(is_not("0123456789\n\r")), newline), find_digits)(source)
}

fn find_digits(source: &str) -> IResult<&str, Vec<char>> {
    many0(find_digit)(source)
}

fn find_digit(source: &str) -> IResult<&str, char> {
    let (source, _) = opt(is_not("0123456789\n\r"))(source)?;
    one_of("0123456789")(source)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn conversion_test_1() {
        let input = "four";
        let left = "4our";
        let right = convert_text(input);
        assert_eq!(left, right);
    }

    #[test]
    fn conversion_test_2() {
        let input = "four4";
        let left = "4our4";
        let right = convert_text(input);
        assert_eq!(left, right);
    }

    #[test]
    fn conversion_test_3() {
        let input = "two1nine";
        let left = "2wo19ine";
        let right = convert_text(input);
        assert_eq!(left, right);
    }

    #[test]
    fn conversion_test_4() {
        let input = "eightwothree";
        let left = "8igh2wo3hree";
        let right = convert_text(input);
        assert_eq!(left, right);
    }

    #[test]
    fn conversion_test_5() {
        let input = "xtwone3four";
        let left = "x2w1ne34our";
        let right = convert_text(input);
        assert_eq!(left, right);
    }

    #[test]
    fn conversion_test_6() {
        let input = "4nineeightseven2";
        let left = "49ine8ight7even2";
        let right = convert_text(input);
        assert_eq!(left, right);
    }

    #[test]
    fn conversion_test_7() {
        let input = "zoneight234";
        let left = "z1n8ight234";
        let right = convert_text(input);
        assert_eq!(left, right);
    }

    #[test]
    fn conversion_test_8() {
        let input = "7pqrstsixteen";
        let left = "7pqrst6ixteen";
        let right = convert_text(input);
        assert_eq!(left, right);
    }

    #[test]
    fn conversion_test_9() {
        let input = "tmmnhlxzpj1eightldxhjnone97";
        let left = "tmmnhlxzpj18ightldxhjn1ne97";
        let right = convert_text(input);
        assert_eq!(left, right);
    }

    #[test]
    fn conversion_test_10() {
        let input = "329";
        let left = "329";
        let right = convert_text(input);
        assert_eq!(left, right);
    }

    #[test]
    fn conversion_test_11() {
        let input = "eighthree";
        let left = "8igh3hree";
        let right = convert_text(input);
        assert_eq!(left, right);
    }

    #[test]
    fn tone_test() {
        let left = solve(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        let right = 281;
        assert_eq!(left, right);
    }

    #[test]
    fn find_digit_tests() {
        assert_eq!(find_digit("alfa1"), Ok(("", '1')));
        assert_eq!(find_digits("bravo2charlie3"), Ok(("", vec!['2', '3'])));
        assert_eq!(find_digits("4delta5echo"), Ok(("echo", vec!['4', '5'])));
        assert_eq!(find_digits("foxtrot6\ngolf7"), Ok(("\ngolf7", vec!['6'])));
        assert_eq!(find_digits("890"), Ok(("", vec!['8', '9', '0'])));
        assert_eq!(find_digits("hotel 1 2"), Ok(("", vec!['1', '2'])));
        assert_eq!(
            find_digits("india 7 9 juliette"),
            Ok((" juliette", vec!['7', '9']))
        );
        assert_eq!(
            find_line_digits("alfa3 bravo 4\n charlie 5"),
            Ok(("", vec![vec!['3', '4'], vec!['5']]))
        );
        assert_eq!(
            find_line_digits("6 delta\n789"),
            Ok(("", vec![vec!['6'], vec!['7', '8', '9']]))
        );
    }
}
