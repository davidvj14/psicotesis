use crate::card_game_extras::GameState;
use leptos::*;

#[server(ProcessCardGame)]
pub async fn process_card_game(state: GameState) -> Result<(), ServerFnError> {
    use crate::app::ssr::*;
    use crate::barrat::get_id_cookie;

    let cookie = get_id_cookie().await?;
    let conn = &mut db().await.unwrap();

    let query = sqlx::query("INSERT INTO cardgame VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(cookie)
        .bind(state.score)
        .bind(state.choices)
        .bind(state.ttf)
        .bind(state.time)
        .bind(state.qs)
        .execute(conn)
        .await;

    println!("{query:?}");

    Ok(())
}
