use crate::models::structs::HousePrice;
use csv::*;
pub fn read_csv(path: &str) -> Vec<HousePrice> {
    let mut rdr = Reader::from_path(path).unwrap(); 
    vec![]
}