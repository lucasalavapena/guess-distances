use std::cmp::PartialEq;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Default, Debug, Clone, PartialEq)]
pub struct Stats {
    pub wrong_by: Vec<f64>,
    running_total: f64,
    pub arithemetic_mean: f64,
}

impl Stats {
    pub fn add_guess(&mut self, value: f64) {
        self.running_total += value.abs();
        self.wrong_by.push(value);
        self.arithemetic_mean = self.running_total / self.wrong_by.len() as f64
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mean_abs_err: f64,
    pub score_mean: f64,
    pub last_guess: Option<f64>,
    pub count: Option<usize>,
}

#[function_component(StatsComponent)]
pub fn stats_component(props: &Props) -> Html {
    let last_guess = match props.last_guess {
        Some(value) => value,
        None => 0.0,
    };

    html! {
        <section class="section">
            <div class="container">
            <h2 class="subtitle is-4"> {"Game Statistics"}</h2>
                <div id="stats-container" class="box">
                    <p><strong>{"Total Guesses: "}</strong> <span id="total-games"> {props.count} </span></p>
                    <p><strong> {"Mean ABS Error : "}</strong> <span id="mean-abs-error"> {format!("{:.2}", props.mean_abs_err)}{" km"}</span></p>
                    <p><strong>{"Last Error: "}</strong> <span id="last-error"> {format!("{:.2}", last_guess)} </span></p>
                    <p><strong>{"Mean Normalised Score: "} </strong> <span id="score"> {format!("{:.2}", props.score_mean)} </span></p>
                </div>
            </div>
        </section>
    }
}
