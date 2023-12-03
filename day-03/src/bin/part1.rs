use regex::Regex;

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

    pub fn line_length(&self) -> usize {
        self.input.lines().nth(0).unwrap().len().try_into().unwrap()
    }

    pub fn markers(&self) -> Vec<usize> {
        let mut markers: Vec<usize> = vec![];
        let re = Regex::new(r"[^0-9.]").unwrap();
        let chars: Vec<char> = self.input_as_line().chars().collect();
        for (i, v) in chars.iter().enumerate() {
            if re.is_match(&v.to_string()) {
                // NOTE: there's a bug here where
                // if the marker as at the start or
                // end of a line it'll push to the wrong
                // position
                markers.push(i - self.line_length() - 1);
                markers.push(i - self.line_length());
                markers.push(i - self.line_length() + 1);
                markers.push(i - 1);
                markers.push(i);
                markers.push(i + 1);
                markers.push(i + self.line_length() - 1);
                markers.push(i + self.line_length());
                markers.push(i + self.line_length() + 1);
            }
        }
        markers
    }

    pub fn numbers(&self) -> Vec<(usize, usize, usize)> {
        let mut numbers: Vec<(usize, usize, usize)> = vec![];
        let re = Regex::new(r"[0-9]").unwrap();
        let chars: Vec<char> = self.input_as_line().chars().collect();
        let mut current_num = (0, 0, 0);
        for (i, v) in chars.iter().enumerate() {
            // refresh on new lines
            if i % self.line_length() == 0 && current_num != (0, 0, 0) {
                numbers.push(current_num);
                current_num = (0, 0, 0);
            }
            if re.is_match(&v.to_string()) {
                let digit = &v.to_string().parse().unwrap();
                if current_num.0 == 0 {
                    current_num.1 = i;
                    current_num.2 = i;
                } else {
                    current_num.2 = i;
                }
                current_num.0 = (current_num.0 * 10) + digit;
            } else if current_num != (0, 0, 0) {
                numbers.push(current_num);
                current_num = (0, 0, 0);
            }
        }
        numbers
    }

    pub fn solve(&self) -> usize {
        let mut value = 0;
        self.numbers().iter().for_each(|num| {
            let mut value_added = false;
            self.markers().iter().for_each(|marker| {
                if (num.1..=num.2).contains(marker) {
                    if value_added == false {
                        value_added = true;
                        value += num.0;
                    }
                }
            });
            ()
        });
        value
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let s = Solver::new_from(input);
    dbg!(s.solve());
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

    #[test]
    fn markers_test() {
        let input = "467..114..
...*......
..35..633.";
        let s = Solver::new_from(input);
        let left = vec![2, 3, 4, 12, 13, 14, 22, 23, 24];
        let right = s.markers();
        assert_eq!(left, right);
    }

    #[test]
    fn numbers_test() {
        let input = "467..114..";
        let s = Solver::new_from(input);
        let left = vec![(467, 0, 2), (114, 5, 7)];
        let right = s.numbers();
        assert_eq!(left, right);
    }

    #[test]
    fn integration_test() {
        let input = "467..114..
...*....23
..35...633
......#...
617*......
.....+.58.
..592.....
11....755.
...$.*....
.664.598..";
        let s = Solver::new_from(input);
        let left = 4361;
        let right = s.solve();
        assert_eq!(left, right);
    }

    #[test]
    fn i2() {
        let input = ".......
...$...
...5...";
        let s = Solver::new_from(input);
        let left = 5;
        let right = s.solve();
        assert_eq!(left, right);
    }

    #[test]
    fn i3() {
        let input = "467..114..
...*....23
...5...633
......#...
617*......
.....+.58.
..592.....
11....755.
...$.*....
.664.598..";
        let s = Solver::new_from(input);
        let left = 4331;
        let right = s.solve();
        assert_eq!(left, right);
    }

    #[test]
    fn i4() {
        let input = "....
.*..
.111
222.";
        let s = Solver::new_from(input);
        let left = 111;
        let right = s.solve();
        assert_eq!(left, right);
    }
}
