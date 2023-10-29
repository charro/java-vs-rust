use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use csv::StringRecord;
use strum_macros::EnumString;

#[derive(Debug, EnumString, Copy, Clone, Eq, Hash, PartialEq)]
enum Ethnicity {
    #[strum(serialize = "ASIAN AND PACIFIC ISLANDER")]
    AsianAndPacificIslander,
    #[strum(serialize = "BLACK NON HISPANIC")]
    BlackNonHispanic,
    #[strum(serialize = "HISPANIC")]
    Hispanic,
    #[strum(serialize = "WHITE NON HISPANIC")]
    WhiteNonHispanic
}

#[derive(Debug)]
struct MostPopularBabyName {
    name: String,
    ethnicity: Ethnicity,
    year: u16
}

impl MostPopularBabyName {
    pub fn from_csv_line(line: &StringRecord) -> Self {
        let name = line[3].to_owned();
        let ethnicity_str = line[2].to_owned();
        let ethnicity = Ethnicity::from_str(ethnicity_str.as_str()).unwrap();
        let year = line[0].to_owned().parse::<u16>().unwrap();

        return MostPopularBabyName{name, ethnicity, year};
    }
}

fn main() {
    let mut most_popular_names_map: HashMap<Ethnicity, Vec<MostPopularBabyName>> = HashMap::new();

    let last_years = HashSet::from(["2017", "2018", "2019"]);
    let reader = csv::Reader::from_path("Popular_Baby_Names_NYC.csv");
    for line in reader.unwrap().records() {
        let line = line.unwrap();
        let gender = &line[1];
        let rank = &line[5];
        let year = &line[0];
        if gender == "FEMALE" && rank == "1" && last_years.contains(year) {
            let most_popular_name = MostPopularBabyName::from_csv_line(&line);
            most_popular_names_map
                .entry(most_popular_name.ethnicity)
                .or_default()
                .push(most_popular_name);
        }
    }

    for element in most_popular_names_map.iter() {
        println!("Element {:?} : {:?}", element.0, element.1);
    }
}
