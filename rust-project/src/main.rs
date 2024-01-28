use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use csv::StringRecord;
use strum_macros::EnumString;

#[derive(Debug)]                // Required to automatically format the value of this enum with println
#[derive(EnumString)]           // Required to construct (serialize) this enum using strings
#[derive(Copy, Clone)]          // Required to copy the enum value instead of moving it
#[derive(Eq, Hash, PartialEq)]  // Required to use the HashMap::entry() API with this enum
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

#[derive(Debug)]                // Required to automatically format the value of this struct with println
struct MostPopularBabyName {
    name: String,
    ethnicity: Ethnicity,
    year: u16
}

impl MostPopularBabyName {
    pub fn from_csv_line(line: &StringRecord) -> Self {
        let name = &line[3];
        let ethnicity_str = &line[2];
        let ethnicity = Ethnicity::from_str(ethnicity_str).expect("Wrong ethnicity");
        let year = line[0].parse::<u16>().expect("Year is not a correct u16 number");

        let name = String::from(name);
        return MostPopularBabyName{name, ethnicity, year};
    }
}

fn main() {

    let last_years = HashSet::from(["2015", "2016", "2017", "2018", "2019"]);
    let reader = csv::Reader::from_path("Popular_Baby_Names_NYC.csv");

    let most_popular_names_list : Vec<MostPopularBabyName> =
        reader.expect("Couldn't open CSV file")
            .records()
            .map(|line| line.expect("Couldn't parse lines of CSV file"))
            .filter(|record| {
                let gender = &record[1];
                let rank = &record[5];
                let year = &record[0];

                gender == "FEMALE" && rank == "1" && last_years.contains(year)
            })
            .map(|record| MostPopularBabyName::from_csv_line(&record))
            .collect();

    let mut most_popular_names_map: HashMap<Ethnicity, Vec<MostPopularBabyName>> = HashMap::new();
    for most_popular_name in most_popular_names_list {
        most_popular_names_map
            .entry(most_popular_name.ethnicity)
            .or_default()
            .push(most_popular_name);
    }

    for (ethnicity, most_popular_names_list) in most_popular_names_map.iter() {
        println!("\nChecking repeated names for ethnicity {:?}: ", ethnicity);
        let mut years_per_name_map: HashMap<&String, Vec<&u16>> = HashMap::new();
        for most_popular_name in most_popular_names_list {
            years_per_name_map
                .entry(&most_popular_name.name)
                .or_default()
                .push(&most_popular_name.year);
        }

        for (name, years) in years_per_name_map {
            if years.len() > 1 {
                println!("The name {} was the most common in more than one year. Years: {:?}", name, years);
            }
        }
    }

}

