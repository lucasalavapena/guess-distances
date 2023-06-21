use crate::geo::City;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GuessState {
    pub expected_answer: f64,
    pub current_input: String,
    pub has_guessed: bool,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CityState {
    pub cities: Vec<City>, // This will store the counter value
    pub first_location_idx: usize,
    pub second_location_idx: usize,
}

// impl Default for CityState {
//     fn default() -> Self {
//         Self {
//             cities: vec![City::default()],
//             ..Default::default()
//         }
//     }
// }

impl CityState {
    pub fn get_city_pair(&self) -> Option<(City, City)> {
        if self.cities.is_empty() {
            return None;
        }
        let first_city = self.cities[self.first_location_idx].clone();
        let second_city = self.cities[self.second_location_idx].clone();
        Some((first_city, second_city))
    }
}
