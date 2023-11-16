use std::rc::Rc;
use std::sync::Arc;

use cities_common::models::City;
use gloo::console::log;

use yew::prelude::*;
use yew::{html, Component, Context, Html};
// use futures::{FutureExt, StreamExt};

use cities_client::client::Client;
use futures_util::future::FutureExt;

use crate::geo::{score, Coord, AngleUnit};

use crate::components::cities::CitiesPair;
use crate::components::stats::{Stats, StatsComponent, StatsType};

use crate::components::settings::{Settings, StettingsComponent};
use crate::state::{self, CitiesState};


// Define the possible messages which can be sent to the component

#[derive(Default, Debug, PartialEq)]
pub enum Mode {
    #[default]
    Playing,
    Setting,
}

pub enum Msg {
    Guess(usize),
    // NextRound,
    InputValue(String),
    NoOp,
    SetCities(state::CitiesState),
    ChangeMode(Mode),
    UpdateSettings(Rc<Settings>),
    ResetStats,
}

#[derive(Default, Debug)]
pub struct App {
    pub cities_state: Option<state::CitiesState>,
    client: Arc<Client>,
    guess_state: state::GuessState,
    settings: Settings,
    mode: Mode,
    offby_km: Stats,
    normalised_score: Stats,
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let client = Arc::new(Client::default());
        let settings = Settings::new(
            Coord {
                latitude: 52.520332,
                longitude: 13.398326,
                type_: AngleUnit::Degrees,
            },            2_000_000,
            1_000_000,)
            ;
        Settings::store(&settings);
        
        let offby_km = Stats::load(StatsType::Offby);
        let normalised_score = Stats::load(StatsType::Normalised);

        let val = CitiesState::new(client.clone(), settings.query.clone());

        // todo look into the map?
        // _ctx.link().send_future( async{let res = val.await.unwrap();Msg::SetCities(res)});
        _ctx.link().send_future(val.map(Msg::SetCities));

        Self {
            cities_state: None,
            client,
            settings,
            offby_km,
            normalised_score,
            ..Default::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            Msg::Guess(value) => {
                let off_by =
                    value as f64 - self.cities_state.as_ref().unwrap().expected_distance / 1000.0;


                let score_val = score(off_by, (self.settings.radius / 1000) as f64);

                self.offby_km.store_guess(off_by, StatsType::Offby);
                self.normalised_score.store_guess(score_val, StatsType::Normalised);

                self.guess_state.has_guessed = true;
                let val =
                    CitiesState::new(self.client.clone(), self.settings.query.clone());
                
                // _ctx.link().send_future( async{let res = val.await.unwrap();Msg::SetCities(res)});
                _ctx.link().send_future(val.map(Msg::SetCities));


                self.guess_state.has_guessed = false;
                self.guess_state.current_input = String::from("");

                true // Return true to cause the displayed change to update
            }
            Msg::SetCities(cities_state) => {
                self.cities_state = Some(cities_state); //#async {.await};
                true
            }
            Msg::InputValue(val) => {
                self.guess_state.current_input = val;
                true
            }
            Msg::ChangeMode(mode) => {
                self.mode = mode;
                true
            }
            Msg::ResetStats => {
                Stats::remove(StatsType::Offby);
                Stats::remove(StatsType::Normalised);

                self.offby_km = Stats::load(StatsType::Offby);
                self.normalised_score = Stats::load(StatsType::Normalised);
                true
            }
            Msg::UpdateSettings(val) => {
                self.settings = (*val).clone();
                Settings::store(&self.settings);
                self.mode = Mode::Playing;
                true
            }
            _ => false,
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
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

        let on_setting_submit: Callback<Rc<Settings>> = ctx.link().callback(move |settings: Rc<Settings>| {
            Msg::UpdateSettings(settings)
        });

        let prev_guess_wrong_by: Option<f64> = self.offby_km.wrong_by.last().copied();

        let first_city: City;
        let second_city: City;

        match &self.cities_state {
            Some(state) => {
                first_city = state.first_city.clone();
                second_city = state.second_city.clone();
            }
            None => {
                first_city = City::default();
                second_city = City::default();
            }
        }

        let reset_stats = ctx.link().callback(|_| Msg::ResetStats);


        let relevant_section = match self.mode {
            Mode::Playing => {
                log!("playing mode");
                html! {
                            <>
                            <div class="settings-icon">
                                <i onclick={ctx.link().callback(|_| Msg::ChangeMode(Mode::Setting))} class="fas fa-cog"></i>
                            </div>
                            <section class="section">
                                <div>
                                    <div class="container">
                                        <CitiesPair city_a={first_city} city_b={second_city} />
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
                                </div>
                        </section>
                    </>
                }
            }
            Mode::Setting => {
                log!("setting mode");

                html! {
                    <StettingsComponent initial_settings={self.settings.clone()} on_settings_submit={on_setting_submit}/>
                }
            }
        };
        html! {
            <body>

                {relevant_section}

                <StatsComponent mean_abs_err={self.offby_km.arithemetic_mean} score_mean={self.normalised_score.arithemetic_mean} count={self.normalised_score.wrong_by.len()} last_guess={prev_guess_wrong_by} reset_click={reset_stats}/>
            </body>

        }
    }
}
