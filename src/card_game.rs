use crate::card_game_extras::GameState;
use leptos::*;

#[server(ProcessCardGame)]
pub async fn process_card_game(state: GameState) -> Result<(), ServerFnError> {
    use crate::app::ssr::*;
    use crate::extras::get_id_cookie;
    use leptos_axum::*;

    let cookie = get_id_cookie().await?;
    let conn = &mut db().await.unwrap();

    let _ = sqlx::query("INSERT INTO cardgame VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(cookie)
        .bind(state.score)
        .bind(state.choices)
        .bind(state.ttf)
        .bind(state.time)
        .bind(state.qs)
        .execute(conn)
        .await?;

    let response = expect_context::<leptos_axum::ResponseOptions>();
    crate::extras::add_cookie("stage", String::from("ending"), &response).await;

    Ok(())
}
