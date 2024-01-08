use std::sync::Arc;

use cities_common::models::City;
use cities_client::client::Client;
use cities_common::queries::{DistQuery, CitiesQuery};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GuessState {
    pub current_input: String,
    pub has_guessed: bool,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CitiesState {
    pub expected_distance: f64, // This will store the counter value
    pub first_city: City,
    pub second_city: City,
}

// 
// // http://127.0.0.1:3000/distance?city_id1=1&city_id2=2

// pub async fn get_random_cities(settings: Settings) {
//     let body = reqwest::get(format!("http://localhost:3000/cities?point=POINT(-0.1276%2051.5074)&radius={setting.radius}&sort_by_random=true&minimum_population={setting.min_population}limit=2"))
//     .await
//     .unwrap().text()
//     .await;

//     println!("body = {:?}", body);
// }

impl CitiesState {
   
    // pub async fn new(client: Arc<Client>, cities_query: CitiesQuery) -> Result<Self, reqwest::Error>  {
    //     let cities = client.get_cities(&cities_query).await?;
    //     let first_city = &cities[0];
    //     let second_city = &cities[1];

    //     let distance_query = DistQuery{
    //         city_id1: first_city.id,
    //         city_id2: second_city.id,

    //     };
    //     let expected_distance = client.get_distance(&distance_query).await?;


    //     Ok::<Self, reqwest::Error>(Self {
    //         first_city: first_city.clone(),
    //         second_city: second_city.clone(),
    //         expected_distance,

    //     })
    // }

    pub async fn new(client: Arc<Client>, cities_query: CitiesQuery) -> Self  {
        let cities = client.get_cities(&cities_query).await.unwrap();
        

        let first_city = &cities[0];
        let second_city = &cities[1];


        let distance_query = DistQuery{
            city_id1: first_city.id,
            city_id2: second_city.id,

        };
        let expected_distance = client.get_distance(&distance_query).await.unwrap();


        Self {
            first_city: first_city.clone(),
            second_city: second_city.clone(),
            expected_distance,

        }
    }

    // pub fn get_new_state(&self) -> impl Future<Output = Result<Self, reqwest::Error>> {

    //     pub first_city: Option<City>,
    //     pub second_city: Option<City>,

    //     if self.cities.is_empty() {
    //         return None;
    //     }
    //     let first_city = self.cities[self.first_location_idx].clone();
    //     let second_city = self.cities[self.second_location_idx].clone();
    //     Some((first_city, second_city))
    // }

}

