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

pub struct Solver {
    pub input: Option<String>,
    maps: HashMap<String, Vec<u32>>,
    map_hashes: HashMap<String, HashMap<u32, u32>>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            input: None,
            maps: HashMap::new(),
            map_hashes: HashMap::new(),
        }
    }

    pub fn get_destination(&mut self, map_type: &str, id: u32) -> u32 {
        if !self.map_hashes.contains_key(map_type) {
            dbg!(format!("Making: {map_type}"));
            let mut new_map_hashes = HashMap::new();
            let entries = self.parse_map_data(map_type).unwrap().1;
            entries.iter().for_each(|entry| {
                let stop_num = entry.1 + entry.2;
                for (indx, update) in (entry.1..stop_num).into_iter().enumerate() {
                    new_map_hashes.insert(update, entry.0 + indx as u32);
                }
            });
            self.map_hashes.insert(map_type.to_string(), new_map_hashes);
        }

        match self.map_hashes.get(map_type).unwrap().get(&id) {
            Some(number) => *number,
            None => id,
        }

        // if !self.maps.contains_key(map_type) {
        //     // dbg!(format!("Making: {map_type}"));
        //     let entries = self.parse_map_data(map_type).unwrap().1;
        //     let max_entries_tuple = entries.iter().max_by_key(|x| x.1 + x.2).unwrap();
        //     let max_entries = max_entries_tuple.1 + max_entries_tuple.2;
        //     let mut new_map: Vec<u32> = vec![];
        //     for slot in 0..=max_entries {
        //         new_map.push(slot)
        //     }
        //     entries.iter().for_each(|entry| {
        //         let stop_num = entry.1 + entry.2;
        //         for (indx, update) in (entry.1..stop_num).into_iter().enumerate() {
        //             new_map[update as usize] = entry.0 + indx as u32
        //         }
        //     });
        //     self.maps.insert(map_type.to_string(), new_map);
        // }
        // if id as usize > self.maps.get(map_type).unwrap().len() {
        //     id
        // } else {
        //     self.maps.get(map_type).unwrap()[id as usize]
        // }
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
