use crate::card_sorting_extras::TestResult;
use leptos::*;

#[server(ProcessCardSorting)]
pub async fn process_card_sorting(result: TestResult) -> Result<(), ServerFnError> {
    use crate::app::ssr::*;
    use crate::barrat::get_id_cookie;

    let cookie = get_id_cookie().await?;

    let conn = &mut db().await.unwrap();

    let _ = sqlx::query("INSERT INTO cardsorting VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
        .bind(cookie)
        .bind(result.score)
        .bind(result.errors)
        .bind(result.perseverations)
        .bind(result.deferred_p)
        .bind(result.m_errors)
        .bind(result.ttf)
        .bind(result.tae)
        .bind(result.time)
        .execute(conn)
        .await;

    Ok(())
}
