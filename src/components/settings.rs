// file:///home/lap/Repos/earth-distance-game/src/components/settings.rs {"mtime":1688246348558,"ctime":1687650328701,"size":6402,"etag":"3ap6kmh6h6kg","orphaned":false,"typeId":""}
// http://localhost:3000/cities?point=POINT(-0.1276%2051.5074)&radius=2500000&sort_by_random=true&minimum_population=500000&limit=2
use gloo::storage::{LocalStorage, Storage};
use std::rc::Rc;

use crate::geo::{AngleUnit, Coord};
use cities_common::queries::CitiesQuery;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::{function_component, html, use_state, Callback, Html, Properties, TargetCast};

use serde::{Deserialize, Serialize};

#[derive(Properties, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Settings {
    pub point: Coord,
    pub radius: usize,
    pub min_population: i32,
    pub query: CitiesQuery,
}
impl Settings {
    const KEY: &'static str = "yew.settings";

    pub fn new(point: Coord, radius: usize, min_population: i32) -> Self {
        let wkt_point = format!("POINT({} {})", point.longitude, point.latitude);

        let query = CitiesQuery {
            radius: Some(radius),
            point: Some(wkt_point),
            minimum_population: Some(min_population),
            limit: Some(2),
            sort_by_random: Some(true),
            ..Default::default()
        };

        Self {
            point,
            radius,
            min_population,
            query,
        }
    }

    pub fn load() -> Self {
        LocalStorage::get(Self::KEY).unwrap_or_default()
    }

    pub fn remove() {
        LocalStorage::delete(Self::KEY);
    }

    pub fn store(&self) {
        let _ = LocalStorage::set(Self::KEY, self);
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new(
            Coord {
                latitude: 52.520332,
                longitude: 13.398326,
                type_: AngleUnit::Degrees,
            },
            2_000_000,
            1_000_000,
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_settings_submit: Callback<Rc<Settings>>,
    pub initial_settings: Settings,
}

// props: &Settings
#[function_component(StettingsComponent)]
pub fn stats_component(props: &Props) -> Html {
    let settings_state = use_state(|| Rc::new(props.initial_settings.clone()));

    let oninput_lat = {
        let settings_state = settings_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let settings: Settings = (**settings_state).clone();
            if let Ok(lat) = input.value().parse::<f64>() { settings_state.set(Rc::new(Settings::new(
                Coord::new(lat, settings.point.longitude),
                settings.radius,
                settings.min_population,
            ))) }
        })
    };

    let oninput_long = {
        let settings_state = settings_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let settings = (**settings_state).clone();
            if let Ok(longitude) = input.value().parse::<f64>() { settings_state.set(Rc::new(Settings::new(
                Coord::new(settings.point.latitude, longitude),
                settings.radius,
                settings.min_population,
            ))) }
        })
    };

    let oninput_radius = {
        let settings_state = settings_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let settings = (**settings_state).clone();
            let radius = input.value().parse::<usize>().unwrap_or_default();
            settings_state.set(Rc::new(Settings::new(
                settings.point,
                radius,
                settings.min_population,
            )))
        })
    };

    let oninput_min_pop = {
        let settings_state = settings_state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let settings = (**settings_state).clone();
            let min_population = input.value().parse::<i32>().unwrap_or_default();
            settings_state.set(Rc::new(Settings::new(
                settings.point,
                settings.radius,
                min_population,
            )));
        })
    };

    let onclick: Callback<MouseEvent> = {
        let settings_submit = props.on_settings_submit.clone();
        let settings_state_c = settings_state.clone();

        Callback::from(move |_e: MouseEvent| settings_submit.emit((*settings_state_c).clone()))
    };

    html! {
        <div>
            <div class="field">
                <label class="label">{"Latitude in Degree"}</label>
                <div class="control">
                    <input class="input" type="number" step="any" min="-90" max="90" value={(settings_state).point.latitude.to_string()}  placeholder="Enter the Latitude in Degrees"  oninput={oninput_lat} />
                </div>
            </div>
            <div class="field">
            <label class="label">{"longitude in Degree"}</label>
            <div class="control">
                <input class="input" type="number" step="any" min="-180" max="180"  value={(settings_state).point.longitude.to_string()} placeholder="Enter the Longitude in Degrees" oninput={oninput_long}/>
            </div>
        </div>

            <div class="field">
                <label class="label">{"Radius [km]"}</label>
                <div class="control">
                    <input class="input" type="number" placeholder="Enter the radius in km" value={(settings_state).radius.to_string()} oninput={oninput_radius} />
                </div>
            </div>

            <div class="field">
                <label class="label">{"Minimum Population [ks]"}</label>
                <div class="control">
                    <input class="input" type="number" placeholder="Enter the minimum population in thousands" value={(settings_state).min_population.to_string()} oninput={oninput_min_pop} />
                </div>
            </div>

            <div class="field">
                <div class="control">
                    <button class="button is-primary" {onclick}>
                        {"Submit"}
                    </button>
                </div>
            </div>
        </div>
    }
}
