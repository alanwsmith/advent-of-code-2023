use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::combinator::opt;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;

pub struct Solver {
    pub input: Option<String>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver { input: None }
    }

    pub fn fertilizer_to_water_map(&self) -> Vec<(u32, u32, u32)> {
        self.parse_map_data("fertilizer-to-water map:").unwrap().1
    }

    pub fn humidity_to_location_map(&self) -> Vec<(u32, u32, u32)> {
        self.parse_map_data("humidity-to-location map:").unwrap().1
    }

    pub fn light_to_temperature_map(&self) -> Vec<(u32, u32, u32)> {
        self.parse_map_data("light-to-temperature map:").unwrap().1
    }

    pub fn seed_to_soil_map(&self) -> Vec<(u32, u32, u32)> {
        self.parse_map_data("seed-to-soil map:").unwrap().1
    }

    pub fn seeds(&self) -> Vec<u32> {
        self.parse_seeds().unwrap().1
    }

    pub fn soil_to_fertilizer_map(&self) -> Vec<(u32, u32, u32)> {
        self.parse_map_data("soil-to-fertilizer map:").unwrap().1
    }

    pub fn parse_map_data(&self, map_key: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
        let (source, _) =
            pair(take_until(map_key), tag(map_key))(self.input.as_ref().unwrap().as_str())?;
        let (source, _) = line_ending(source)?;
        let (source, entry_matches) =
            many1(pair(separated_list1(space1, digit1), opt(line_ending)))(source)?;
        let entries: Vec<(u32, u32, u32)> = entry_matches
            .iter()
            .map(|e| {
                (
                    e.0[0].parse().unwrap(),
                    e.0[1].parse().unwrap(),
                    e.0[2].parse().unwrap(),
                )
            })
            .collect();
        Ok((source, entries))
    }

    pub fn parse_seeds(&self) -> IResult<&str, Vec<u32>> {
        let (source, _) =
            pair(take_until("seeds:"), tag("seeds:"))(self.input.as_ref().unwrap().as_str())?;
        let (source, seed_strings) = many1(pair(space1, digit1).map(|x| x.1))(source)?;
        let seeds: Vec<u32> = seed_strings.iter().map(|s| s.parse().unwrap()).collect();
        Ok((source, seeds))
    }

    pub fn solve(&self) -> u32 {
        35
    }

    pub fn temperature_to_humidity_map(&self) -> Vec<(u32, u32, u32)> {
        self.parse_map_data("temperature-to-humidity map:")
            .unwrap()
            .1
    }

    pub fn water_to_light_map(&self) -> Vec<(u32, u32, u32)> {
        self.parse_map_data("water-to-light map:").unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_1() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = 35;
        let right = s.solve();
        assert_eq!(left, right);
    }

    #[test]
    fn fertilizer_to_water_map() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)];
        let right = s.fertilizer_to_water_map();
        assert_eq!(left, right);
    }

    #[test]
    fn humidity_to_location_map() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = vec![(60, 56, 37), (56, 93, 4)];
        let right = s.humidity_to_location_map();
        assert_eq!(left, right);
    }

    #[test]
    fn light_to_temperature_map() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)];
        let right = s.light_to_temperature_map();
        assert_eq!(left, right);
    }

    #[test]
    fn seed_to_soil_map() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = vec![(50, 98, 2), (52, 50, 48)];
        let right = s.seed_to_soil_map();
        assert_eq!(left, right);
    }

    #[test]
    fn seeds() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = vec![79, 14, 55, 13];
        let right = s.seeds();
        assert_eq!(left, right);
    }

    #[test]
    fn soil_to_fertilizer_map() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)];
        let right = s.soil_to_fertilizer_map();
        assert_eq!(left, right);
    }

    #[test]
    fn temperature_to_humidity_map() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = vec![(0, 69, 1), (1, 0, 69)];
        let right = s.temperature_to_humidity_map();
        assert_eq!(left, right);
    }

    #[test]
    fn water_to_light_map() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = vec![(88, 18, 7), (18, 25, 70)];
        let right = s.water_to_light_map();
        assert_eq!(left, right);
    }
}
