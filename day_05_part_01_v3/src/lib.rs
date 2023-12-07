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
use std::collections::HashMap;
use std::ops::Range;

pub struct Solver {
    pub input: Option<String>,
    range_maps: HashMap<String, Vec<(Range<u32>, i32)>>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            input: None,
            range_maps: HashMap::new(),
        }
    }

    pub fn get_destination(&mut self, map_type: &str, id: u32) -> u32 {
        if !self.range_maps.contains_key(map_type) {
            let entries = self.parse_map_data(map_type).unwrap().1;
            let mut ranges: Vec<(Range<u32>, i32)> = vec![];
            entries.iter().for_each(|entry| {
                let range_start = entry.1;
                let range_end = entry.1 + entry.2;
                let offset: i32 = entry.1 as i32 - entry.0 as i32;
                let new_range = range_start..range_end;
                ranges.push((new_range, offset));
            });
            self.range_maps.insert(map_type.to_string(), ranges);
        }

        let mut return_value = id;

        self.range_maps
            .get(map_type)
            .unwrap()
            .iter()
            .for_each(|rng| {
                if rng.0.contains(&id) {
                    let new_return_value = id as i32 - rng.1;
                    return_value = new_return_value as u32;
                }
            });

        return_value
    }

    pub fn get_seed_location(&mut self, id: u32) -> u32 {
        let soil_id = self.get_destination("seed-to-soil", id);
        let fertilizer_id = self.get_destination("soil-to-fertilizer", soil_id);
        let water_id = self.get_destination("fertilizer-to-water", fertilizer_id);
        let light_id = self.get_destination("water-to-light", water_id);
        let temperature_id = self.get_destination("light-to-temperature", light_id);
        let humidity_id = self.get_destination("temperature-to-humidity", temperature_id);
        let location_id = self.get_destination("humidity-to-location", humidity_id);
        location_id
    }

    pub fn seeds(&self) -> Vec<u32> {
        self.parse_seeds().unwrap().1
    }

    pub fn parse_map_data(&self, map_key: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
        let (source, _) =
            pair(take_until(map_key), tag(map_key))(self.input.as_ref().unwrap().as_str())?;
        let (source, _) = tag(" map:")(source)?;
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

    pub fn solve(&mut self) -> u32 {
        self.seeds()
            .into_iter()
            .map(|id| self.get_seed_location(id))
            .min()
            .unwrap()
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
    fn get_destination_without_changes() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = 14;
        let right = s.get_destination("seed-to-soil", 14);
        assert_eq!(left, right);
    }

    #[test]
    fn get_destination_from_beyond_last_mod() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = 1000;
        let right = s.get_destination("seed-to-soil", 1000);
        assert_eq!(left, right);
    }

    #[test]
    fn get_destination_without_changes_2() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = 13;
        let right = s.get_destination("seed-to-soil", 13);
        assert_eq!(left, right);
    }

    #[test]
    fn get_destination_with_changes() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = 81;
        let right = s.get_destination("seed-to-soil", 79);
        assert_eq!(left, right);
    }

    #[test]
    fn get_destination_with_changes_2() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = 57;
        let right = s.get_destination("seed-to-soil", 55);
        assert_eq!(left, right);
    }

    #[test]
    fn get_seed_location() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = 82;
        let right = s.get_seed_location(79);
        assert_eq!(left, right);
    }

    #[test]
    fn get_seed_location_2() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = 43;
        let right = s.get_seed_location(14);
        assert_eq!(left, right);
    }

    #[test]
    fn get_seed_location_3() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = 86;
        let right = s.get_seed_location(55);
        assert_eq!(left, right);
    }

    #[test]
    fn get_seed_location_4() {
        let mut s = Solver::new();
        s.input = Some(include_str!("../input-test.txt").to_string());
        let left = 35;
        let right = s.get_seed_location(13);
        assert_eq!(left, right);
    }
}
