// file:///home/lap/Repos/earth-distance-game/src/components/cities.rs {"mtime":1687780968721,"ctime":1687091487676,"size":1612,"etag":"3aod0n1b01l0","orphaned":false,"typeId":""}
// use crate::geo::City;
use cities_common::models::City;
use emojis::Emoji;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub city_a: City,
    pub city_b: City,
}

fn get_flag_emoji(country_code: &str) -> Option<String> {
    let code_points: Option<Vec<char>> = country_code
        .to_uppercase()
        .chars()
        .map(|c| (127397 + c as u32).try_into().ok())
        .collect();

    code_points.map(|cp| cp.iter().collect())
}

fn get_flag_from_country_code(country_code: String) -> &'static Emoji {
    let res = get_flag_emoji(&country_code);
    match res {
        Some(val_flag) => match emojis::get(&val_flag) {
            Some(emoji_val) => emoji_val,
            None => emojis::get("🤌").unwrap(),
        },
        None => emojis::get("🤌").unwrap(),
    }
}

#[function_component]
pub fn CitiesPair(props: &Props) -> Html {
    let flag_a = get_flag_from_country_code(props.city_a.iso2.clone());
    let flag_b = get_flag_from_country_code(props.city_b.iso2.clone());

    html! {
            <div id="game-container" class="box custom-box">
                <div id="question-container" class="mb-4">
                <h2 class="subtitle is-4"> {"Guess the distance:"}</h2>
                    <p id="question-text" class="title is-3"> {flag_a} {" "} {props.city_a.name.clone()}{", "}{props.city_a.country.clone()} <a href="https://example.com">{'🌐'}</a></p>
                    <p id="question-text" class="title is-3"> {flag_b} {" "} {props.city_b.name.clone()}{", "}{props.city_b.country.clone()} <a href="https://example.com">{'🌐'}</a></p>

                </div>
            </div>
    }
}
