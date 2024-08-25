use leptos::*;
#[cfg(feature = "ssr")]
use leptos_axum::*;
#[cfg(feature = "ssr")]
use std::num::ParseIntError;
#[cfg(feature = "ssr")]
use axum_extra::extract::cookie::{CookieJar, Cookie, SameSite};
#[cfg(feature = "ssr")]
use axum::response::IntoResponse;
use cookie::time::Duration;


use crate::app::Stage;


#[cfg(feature = "ssr")]
pub async fn get_id_cookie() -> Result<i32, ServerFnError> {
    use axum::http::HeaderMap;
    use axum::extract::FromRequestParts;
    use leptos_axum::extract;
    use axum_extra::extract::cookie::{CookieJar, Cookie};

    let mut hm = extract().await?;
    let jar = CookieJar::from_request_parts(&mut hm, &()).await.unwrap();
    if let Ok(value) = jar.get("p_id").unwrap().value().parse::<i32>() {
        return Ok(value);
    } else {
        return Err(ServerFnError::Request(String::from("Bad p_id cookie")));
    }
}


#[cfg(feature = "hydrate")]
pub fn read_cookie(name: &str) -> Option<String> {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlDocument;

    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.dyn_into::<HtmlDocument>().ok())
        .and_then(|d| d.cookie().ok())
        .and_then(|cookies| {
            logging::log!("read_cookie cookies {}", cookies);
            cookies
                .split(';')
                .find_map(|cookie| {
                    let (key, val) = cookie.trim().split_once('=')?;
                    if key == name {
                        logging::log!("read_cookie matched {} : {}", key, val);
                        Some(val.to_string())
                    } else {
                        None
                    }
                })
        })
}

#[cfg(not(feature = "hydrate"))]
pub fn read_cookie(_name: &str) -> Option<String> {
    None
}

#[cfg(feature = "ssr")]
pub async fn add_cookie(key: &'static str, val: String, response: &ResponseOptions) {
    use http::{header, HeaderValue};
    use axum::extract::FromRequestParts;
    use leptos_axum::extract;

    let mut hm = extract().await.unwrap();
    let jar = CookieJar::from_request_parts(&mut hm, &()).await.unwrap();

    let cookie = Cookie::build((key, val))
        .path("/")
        .secure(true)
        .http_only(false)
        .same_site(SameSite::Lax)
        .max_age(Duration::minutes(3600));
    response.append_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );
}

#[cfg(feature = "hydrate")]
pub fn get_stage_from_cookie() -> Stage {
    let cookie = read_cookie("stage");
    logging::log!("setter cookie: {:?}", cookie);
    match cookie.unwrap_or(String::new()).as_str() {
        "barrat" => Stage::Barrat,
        "card_sorting" => Stage::CardSorting,
        "card_game" => Stage::CardGame,
        "ending" => Stage::Ending,
        "void" => Stage::Void,
        _ => Stage::Questions
    }
}

#[cfg(not (feature = "hydrate"))]
pub fn get_stage_from_cookie() -> Stage {
    Stage::Questions
}
