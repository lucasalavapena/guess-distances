// file:///home/lap/Repos/earth-distance-game/src/components/stats.rs {"mtime":1688247602794,"ctime":1687269124427,"size":2836,"etag":"3ap6mg0h32tf","orphaned":false,"typeId":""}
use std::cmp::PartialEq;
use web_sys::MouseEvent;
use yew::{function_component, html, Html, Properties, Callback};

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum StatsType {
    #[default]
    Offby,
    Normalised,
}

#[derive(Properties, Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Stats {
    pub wrong_by: Vec<f64>,
    running_total: f64,
    pub arithemetic_mean: f64,
}

impl Stats {
    const KEY: &'static str = "yew.stats";

    // caching?
    fn get_key(mode: StatsType) -> String {
        let mode_str = match mode {
            StatsType::Offby => "offby",
            StatsType::Normalised => "normalised",
        };
        format!("{}.{}", Self::KEY, mode_str)
    }


    pub fn load(mode: StatsType) -> Self {
        let key = Self::get_key(mode);
        LocalStorage::get(key).unwrap_or_default()
    }

    pub fn remove(mode: StatsType) {
        let key = Self::get_key(mode);
        LocalStorage::delete(key);
    }


    pub fn store_guess(&mut self, value: f64, mode: StatsType) {
        let key = Self::get_key(mode);

        self.running_total += value.abs();
        self.wrong_by.push(value);
        self.arithemetic_mean = self.running_total / self.wrong_by.len() as f64;
        let _ = LocalStorage::set(key, self);
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mean_abs_err: f64,
    pub score_mean: f64,
    pub last_guess: Option<f64>,
    pub count: Option<usize>,
    pub reset_click: Callback<MouseEvent>,
}

#[function_component(StatsComponent)]
pub fn stats_component(props: &Props) -> Html {
    let last_guess = props.last_guess.unwrap_or(0.0);
    let onclick = props.reset_click.reform(|x| x);


    html! {
        <section class="section">
            <div class="container">
            <h2 class="subtitle is-4"> {"Game Statistics"}</h2>
                <div id="stats-container" class="box">
                    <p><strong>{"Total Guesses: "}</strong> <span id="total-games"> {props.count} </span></p>
                    <p><strong> {"Mean ABS Error : "}</strong> <span id="mean-abs-error"> {format!("{:.2}", props.mean_abs_err)}{" km"}</span></p>
                    <p><strong>{"Last Error: "}</strong> <span id="last-error"> {format!("{:.2}", last_guess)}{" km"}</span></p>
                    <p><strong>{"Mean Normalised Score: "} </strong> <span id="score"> {format!("{:.2}", props.score_mean)} </span></p>
                    <p style="text-align:center"> 
                        <button class="button is-danger is-small" onclick={onclick}>
                            {"Reset"}
                        </button>
                    </p>
                </div>

            </div>
        </section>
    }
}
