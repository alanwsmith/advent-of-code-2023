use nom::bytes::complete::is_not;
use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::sequence::pair;
use nom::IResult;

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(source: &str) -> u32 {
    let (_, results) = find_line_digits(source).unwrap();
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
    #[ignore]
    fn tone_test() {
        let left = solve(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        let right = 142;
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
