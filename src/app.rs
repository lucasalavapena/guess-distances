use rand::random;
use yew::prelude::*;
use yew::{html, use_state, Callback, Component, Context, Html};
use gloo::console::log;

use crate::components::cities::CitiesPair;
use crate::geo::{abs_dist, compute_distance, score, AngleUnit, City, Coord, WGS84, load_cities_from_str, load_cities_from_csv};
use crate::state;
use crate::stats::{Stats, StatsRow, ScoreRow};
use crate::traits::ChangeAngle;

// Define the possible messages which can be sent to the component

pub enum Msg {
    Guess(usize),
    NextRound,
    InputValue(String),
    NoOp,
}

#[derive(Default, Debug, PartialEq)]
pub struct App {
    pub city_state: state::CityState,
    guess_state: state::GuessState,
    stats_km: Stats,
    stats_score: Stats,
}

impl App {
    fn compute_new_triplet(Citys: &Vec<City>) -> (usize, usize, f64) {
        let first_location_idx = random::<usize>() % Citys.len();
        let second_location_idx = random::<usize>() % Citys.len();

        let first_city = &Citys[first_location_idx].coordinates;
        let second_city = &Citys[second_location_idx].coordinates;
        let approximate_answer = compute_distance(first_city, second_city, WGS84) / 1000.0;

        (
            first_location_idx,
            second_location_idx,
            approximate_answer,
        )
    }
}

// impl Default for App {
//     fn default() -> Self {
//         let Citys: Vec<City> = vec![];

//         Self {
//             Citys,
//             first_location_idx: 0,
//             second_location_idx: 0,
//             expected_answer: 0,
//             current_input: "".to_string(),
//             stats: Stats::default(),
//         }
//     }
// }

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let Citys_from_csv = load_cities_from_str(
            include_str!("../static/capitals.csv"),
        );
        // 

        // let Citys_from_csv: Result<Vec<City>, ()> = Ok(vec![
        //     City {
        //         coordinates: Coord {
        //             latitude: 9.55,
        //             longitude: 44.05,
        //             type_: AngleUnit::Degrees,
        //         },
        //         country_name: "Somaliland".to_string(),
        //         city_name: "Hargeisa".to_string(),
        //         country_code: None,
        //         continent_name: "Africa".to_string(),
        //     },
        //     City {
        //         coordinates: Coord {
        //             latitude: -54.283333,
        //             longitude: -36.5,
        //             type_: AngleUnit::Degrees,
        //         },
        //         country_name: "South Georgia and South Sandwich Islands".to_string(),
        //         city_name: "King Edward Point".to_string(),
        //         country_code: Some(['G', 'S']),
        //         continent_name: "Antarctica".to_string(),
        //     },
        //     City {
        //         coordinates: Coord {
        //             latitude: -49.35,
        //             longitude: 70.216667,
        //             type_: AngleUnit::Degrees,
        //         },
        //         country_name: "French Southern and Antarctic Lands".to_string(),
        //         city_name: "Port-aux-Français".to_string(),
        //         country_code: Some(['T', 'F']),
        //         continent_name: "Antarctica".to_string(),
        //     },
        //     City {
        //         coordinates: Coord {
        //             latitude: 31.766666666666666,
        //             longitude: 35.233333,
        //             type_: AngleUnit::Degrees,
        //         },
        //         country_name: "Palestine".to_string(),
        //         city_name: "Jerusalem".to_string(),
        //         country_code: Some(['P', 'S']),
        //         continent_name: "Asia".to_string(),
        //     },
        //     City {
        //         coordinates: Coord {
        //             latitude: 60.116667,
        //             longitude: 19.9,
        //             type_: AngleUnit::Degrees,
        //         },
        //         country_name: "Aland Islands".to_string(),
        //         city_name: "Mariehamn".to_string(),
        //         country_code: Some(['A', 'X']),
        //         continent_name: "Europe".to_string(),
        //     },
        //     City {
        //         coordinates: Coord {
        //             latitude: -0.5477,
        //             longitude: 166.920867,
        //             type_: AngleUnit::Degrees,
        //         },
        //         country_name: "Nauru".to_string(),
        //         city_name: "Yaren".to_string(),
        //         country_code: Some(['N', 'R']),
        //         continent_name: "Australia".to_string(),
        //     },
        //     City {
        //         coordinates: Coord {
        //             latitude: 18.0731,
        //             longitude: -63.0822,
        //             type_: AngleUnit::Degrees,
        //         },
        //         country_name: "Saint Martin".to_string(),
        //         city_name: "Marigot".to_string(),
        //         country_code: Some(['M', 'F']),
        //         continent_name: "North America".to_string(),
        //     },
        //     City {
        //         coordinates: Coord {
        //             latitude: -9.166667,
        //             longitude: -171.833333,
        //             type_: AngleUnit::Degrees,
        //         },
        //         country_name: "Tokelau".to_string(),
        //         city_name: "Atafu".to_string(),
        //         country_code: Some(['T', 'K']),
        //         continent_name: "Australia".to_string(),
        //     },
        // ]);
        if let Ok(cities) = Citys_from_csv {
            let cities_rad: Vec<City> = cities.iter().map(|c: &City| c.change_unit()).collect();            let no_captials = cities.len();
            log!("Successfully loaded {} Citys", no_captials);

            let (first_location_idx, second_location_idx, expected_answer) =
                App::compute_new_triplet(&cities_rad);
            Self {
                city_state: state::CityState {
                    cities: cities_rad,
                    first_location_idx,
                    second_location_idx,
                },
                guess_state: state::GuessState {
                    expected_answer,
                    ..Default::default()
                },
                ..Default::default()
            }
        } else {
            Self::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Guess(value) => {
                // color guess green or read and update stas
                // compute_distance(coord_a: Coord, coord_b: Coord, coord_system: CoordSystem)

                let off_by = value as f64 -  self.guess_state.expected_answer;
                // abs_dist(value, self.guess_state.expected_answer);
                // log!("off by km:");
                // log!(off_by);
                let score_val = score(off_by); 
                log!(score_val);

                self.stats_km.add_guess(off_by);
                self.stats_score.add_guess(score_val);


                self.guess_state.has_guessed = true;
                true // Return true to cause the displayed change to update
            }
            Msg::NextRound => {
                (
                    self.city_state.first_location_idx,
                    self.city_state.second_location_idx,
                    self.guess_state.expected_answer,
                ) = App::compute_new_triplet(&self.city_state.cities);
                self.guess_state.has_guessed = false;

                true
            }
            Msg::InputValue(val) => {
                self.guess_state.current_input = val;
                true
            }
            _ => false,
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let (first_city, second_city) = self.city_state.get_city_pair().unwrap();
        let input_value = self.guess_state.current_input.clone();

        let onsubmit = {
            ctx.link().callback(move |e: SubmitEvent| {
                e.prevent_default(); /* Prevent event propagation */
                // handle this
                match input_value.parse() {
                    Ok(value) => Msg::Guess(value),
                    Err(err) => {log!("input is not a valid number"); Msg::NoOp},
                }
            })
        };

        let oninput = {
            ctx.link().callback(|e: yew::InputEvent| {
                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                Msg::InputValue(input.value())
            })
        };

        let prev_guess_wrong_by: Option<f64> = self.stats_km.wrong_by.last().copied();

        html! {
            //
            <div >
                <div class="container">
                    <CitiesPair city_a={first_city} city_b={second_city} />
                </div>

                <form onsubmit={onsubmit}>
                    <label for="guess">{"Enter your guess (km):"}</label>
                    <input
                        id="guess"
                        type="number"
                        value={self.guess_state.current_input.clone()}
                        oninput={oninput}
                    />
                <button type="submit">{"Submit"}</button>
                </form>
                <button type="submit" onclick={ctx.link().callback(|_| Msg::NextRound)}>{"Next"}</button>
                <StatsRow arithemetic_mean={self.stats_km.arithemetic_mean} count={self.stats_km.wrong_by.len()} last_guess={prev_guess_wrong_by}/>
                <ScoreRow arithemetic_mean={self.stats_score.arithemetic_mean}/>
            </div>


        }
    }
}
