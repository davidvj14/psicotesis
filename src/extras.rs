use leptos::*;
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
