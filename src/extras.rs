use leptos::*;
#[cfg(feature = "ssr")]
use std::num::ParseIntError;

#[cfg(feature = "ssr")]
fn parse_id_cookie(cookie: &str) -> Result<i32, ParseIntError> {
    return Ok(cookie[5..].parse()?);
}

#[cfg(feature = "ssr")]
pub async fn get_id_cookie() -> Result<i32, ServerFnError> {
    use axum::http::HeaderMap;
    use leptos_axum::extract;

    let hm: HeaderMap = extract().await?;
    if let Some(cookie_header) = hm.get("cookie") {
        if let Ok(cookie_header_str) = cookie_header.to_str() {
            let cookie = parse_id_cookie(cookie_header_str)?;
            println!("{cookie:?}");
            return Ok(cookie);
        }
    }
    Err(ServerFnError::MissingArg(String::from("Bad p_id cookie")))
}
