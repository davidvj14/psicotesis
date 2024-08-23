use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
#[cfg(feature = "ssr")]
use std::num::ParseIntError;

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
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.dyn_into::<HtmlDocument>().ok())
        .and_then(|d| d.cookie().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find_map(|cookie| {
                    let (key, val) = cookie.trim().split_once('=')?;
                    if key == name {
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
