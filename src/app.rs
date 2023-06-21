use gloo::console::log;
use rand::random;
use yew::prelude::*;
use yew::{html, Component, Context, Html};

use crate::components::cities::CitiesPair;
use crate::components::stats::{Stats, StatsComponent};
use crate::geo::{
    compute_distance, load_cities_from_str, score, City, WGS84,
};
use crate::state;
use crate::traits::ChangeAngle;

// Define the possible messages which can be sent to the component

pub enum Msg {
    Guess(usize),
    // NextRound,
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

        (first_location_idx, second_location_idx, approximate_answer)
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let Citys_from_csv = load_cities_from_str(include_str!("../static/capitals.csv"));

        if let Ok(cities) = Citys_from_csv {
            let cities_rad: Vec<City> = cities.iter().map(|c: &City| c.change_unit()).collect();
            let no_captials = cities.len();
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
                let off_by = value as f64 - self.guess_state.expected_answer;

                let score_val = score(off_by);

                self.stats_km.add_guess(off_by);
                self.stats_score.add_guess(score_val);

                self.guess_state.has_guessed = true;

                (
                    self.city_state.first_location_idx,
                    self.city_state.second_location_idx,
                    self.guess_state.expected_answer,
                ) = App::compute_new_triplet(&self.city_state.cities);
                self.guess_state.has_guessed = false;
                self.guess_state.current_input = String::from("");

                true // Return true to cause the displayed change to update
            }
            // Msg::NextRound => {
            //     (
            //         self.city_state.first_location_idx,
            //         self.city_state.second_location_idx,
            //         self.guess_state.expected_answer,
            //     ) = App::compute_new_triplet(&self.city_state.cities);
            //     self.guess_state.has_guessed = false;

            //     true
            // }
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
                    Err(_err) => {
                        log!("input is not a valid number");
                        Msg::NoOp
                    }
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
            <body>
            <section class="section">
                    <div>
                        <div class="container">
                            <CitiesPair city_a={first_city} city_b={second_city} />
                        </div>
                        <div id="input-container" class="field has-addons">
                            // <div class="control is-expanded">
                            //     <input
                            //         id="city-input"
                            //         class="input is-medium"
                            //         placeholder={"Enter your guess in km:"}
                            //         type="number"
                            //         value={self.guess_state.current_input.clone()}
                            //         oninput={oninput}
                            //     />
                            // </div>
                            // <div class="control">
                            //     <button id="submit-button" class="button is-primary is-medium">{"Submit"}</button>
                            // </div>
                        </div>

                        <form onsubmit={onsubmit} class="form has-addons">
                            <div class="control is-expanded">
                                <input
                                    id="city-input"
                                    class="input is-medium"
                                    placeholder={"Enter your guess in km:"}
                                    type="number"
                                    value={self.guess_state.current_input.clone()}
                                    oninput={oninput}
                                />
                            </div>
                        </form>
                        // <button type="submit" onclick={ctx.link().callback(|_| Msg::NextRound)}>{"Next"}</button>
                    </div>
                </section>

                <StatsComponent mean_abs_err={self.stats_km.arithemetic_mean} score_mean={self.stats_score.arithemetic_mean} count={self.stats_km.wrong_by.len()}last_guess={prev_guess_wrong_by} />
            </body>

        }
    }
}
