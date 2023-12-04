use regex::Regex;

struct Solver {
    input: String,
}

#[derive(Debug, PartialEq)]
struct Gear {
    indexes: Vec<usize>,
    ratios: Vec<usize>,
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

    pub fn markers(&self) -> Vec<Gear> {
        let mut markers: Vec<Gear> = vec![];
        let re = Regex::new(r"[*]").unwrap();
        let chars: Vec<char> = self.input_as_line().chars().collect();
        for (i, v) in chars.iter().enumerate() {
            if re.is_match(&v.to_string()) {
                // NOTE: there's a bug here where
                // if the marker as at the start or
                // end of a line it'll push to the wrong
                // position
                let g = Gear {
                    ratios: vec![],
                    indexes: vec![
                        i - self.line_length() - 1,
                        i - self.line_length(),
                        i - self.line_length() + 1,
                        i - 1,
                        i,
                        i + 1,
                        i + self.line_length() - 1,
                        i + self.line_length(),
                        i + self.line_length() + 1,
                    ],
                };
                markers.push(g);
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
        let mut start_markers = self.markers();
        start_markers.iter_mut().for_each(|marker| {
            self.numbers().iter().for_each(|num| {
                let mut value_added = false;
                marker.indexes.iter().for_each(|index| {
                    if (num.1..=num.2).contains(index) {
                        if value_added == false {
                            value_added = true;
                            marker.ratios.push(num.0);
                        }
                    }
                });
            });
        });

        dbg!(&start_markers);

        let value = start_markers.iter().fold(0, |acc, x| {
            if x.ratios.len() as u32 == 2 {
                acc + (&x.ratios[0] * &x.ratios[1])
            } else {
                acc
            }
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
    fn line_length_test() {
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
        let left = vec![Gear {
            indexes: vec![2, 3, 4, 12, 13, 14, 22, 23, 24],
            ratios: vec![],
        }];
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
    fn integration1() {
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
        let s = Solver::new_from(input);
        let left = 467835;
        let right = s.solve();
        assert_eq!(left, right);
    }
}
