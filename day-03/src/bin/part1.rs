#![allow(dead_code)]
#![allow(unused_variables)]

struct Solver {
    input: String,
}

impl Solver {
    pub fn new_from(source: &str) -> Solver {
        Solver {
            input: source.to_string(),
        }
    }

    pub fn input_as_line(&self) -> String {
        self.input.lines().collect()
    }

    pub fn line_length(&self) -> u32 {
        self.input.lines().nth(0).unwrap().len().try_into().unwrap()
    }

    // pub fn markers(&self) -> u32 {}
}

fn main() {
    println!("Hello, world!");
}

fn solve(source: &str) -> u32 {
    4361
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_as_lines_test() {
        let input = "467..114..
...*......";
        let s = Solver::new_from(input);
        let left = "467..114.....*......".to_string();
        let right = s.input_as_line();
        assert_eq!(left, right);
    }

    #[test]
    fn get_line_length() {
        let input = "467..114..
...*......";
        let s = Solver::new_from(input);
        let left = 10;
        let right = s.line_length();
        assert_eq!(left, right);
    }

    // #[test]
    // fn get_markers() {
    //     let input = "467..114..
    // ...*......
    // ..35..633.";
    //     let s = Solver::new_from(input);
    //     let left = vec![3, 4, 5, 13, 14, 15, 23, 24, 25];
    //     let right = s.markers();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn tone_test() {
    //     assert_eq!(1, 2);
    // }

    #[test]
    fn integration_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let left = 4361;
        let right = solve(input);
        assert_eq!(left, right);
    }
}

