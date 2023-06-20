
use std::cmp::PartialEq;
use num_traits::{Num, Float, Signed};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Default, Debug, Clone, PartialEq)]
pub struct Stats {
    pub wrong_by: Vec<f64>,
    running_total: f64,
    pub arithemetic_mean: f64,
}

impl Stats{
    pub fn add_guess(&mut self, value: f64) {
        self.running_total += value.abs();
        self.wrong_by.push(value);
        self.arithemetic_mean = self.running_total / self.wrong_by.len() as f64
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub arithemetic_mean: Option<f64>,
    pub last_guess: Option<f64>,
    pub count: Option<usize>,
}

#[function_component(StatsRow)]
pub fn stats_component(props: &Props) -> Html {

    let last_guess = match props.last_guess {
        Some(value) => value,
        None => 0.0,
    };

    html! {
        <>
            <div class="row">
                <div class="col"> {"Mean: "}{props.arithemetic_mean}{" km"} </div>
                <div class="col"> {"count: "}{props.count} </div>
                <div class="col"> {"Last guess was wrong by: "}{last_guess}{" km"} </div>
            </div>

        </>
    }
}


#[derive(Properties, PartialEq)]
pub struct ScoreProps {
    pub arithemetic_mean: f64,
}

#[function_component(ScoreRow)]
pub fn score_component(props: &ScoreProps) -> Html {
    html! {
        <>
            <div class="row">
                <div class="col"> {"Arithmetic mean: "}{props.arithemetic_mean} </div>
            </div>

        </>
    }
}