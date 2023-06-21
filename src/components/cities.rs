use crate::geo::City;
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

fn get_flag_from_country_code(country_code: Option<[char; 2]>) -> &'static Emoji {
    match country_code {
        Some(cc) => {
            let cc_str: String = cc.iter().collect::<String>();
            let res = get_flag_emoji(&cc_str);
            // log!(res.unwrap());
            match res {
                Some(val_flag) => match emojis::get(&val_flag) {
                    Some(emoji_val) => emoji_val,
                    None => emojis::get("🤌").unwrap(),
                },
                None => emojis::get("🤌").unwrap(),
            }
        }
        None => emojis::get("🤌").unwrap(),
    }
}

#[function_component]
pub fn CitiesPair(props: &Props) -> Html {
    let flag_a = get_flag_from_country_code(props.city_a.country_code);
    let flag_b = get_flag_from_country_code(props.city_b.country_code);

    html! {
            <div id="game-container" class="box custom-box">
                <div id="question-container" class="mb-4">
                <h2 class="subtitle is-4"> {"Guess the distance:"}</h2>
                    <p id="question-text" class="title is-3"> {flag_a} {" "} {props.city_a.full_name().clone()}</p>
                    <p id="question-text" class="title is-3"> {flag_b} {" "} {props.city_b.full_name().clone()}</p>

                </div>
            </div>
    }
}
