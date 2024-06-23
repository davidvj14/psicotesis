use crate::error_template::{AppError, ErrorTemplate};
use crate::questions_components::Questions;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::time::Duration;

#[cfg(feature = "ssr")]
pub mod ssr {
    use dotenvy::dotenv;
    use leptos::ServerFnError;
    use sqlx::{Connection, PgConnection};

    pub async fn db() -> Result<PgConnection, ServerFnError> {
        dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Ok(PgConnection::connect(&db_url).await?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage {
    Questions,
    Barrat,
    CardSorting,
    CardGame,
    CardGameQs,
    Thanks,
}

#[component]
fn Tests() -> impl IntoView {
    //let (stage, stage_setter) = create_signal(Stage::Questions); :(
    let questions = create_rw_signal(true);
    let barrat = create_rw_signal(false);
    let card_sorting = create_rw_signal(false);
    let card_game = create_rw_signal(false);
    let card_gameqs = create_rw_signal(false);
    let thanks = create_rw_signal(false);

    view! {
        <AnimatedShow
            when=questions
            show_class = "fade-in-1000"
            hide_class = "fade-out-1000"
            hide_delay = Duration::from_millis(1000)
        >
            <Questions questions_signal=questions barrat_signal=barrat/>
        </AnimatedShow>
        <AnimatedShow
            when=barrat
            show_class = "fade-in-1000"
            hide_class = "fade-out-1000"
            hide_delay = Duration::from_millis(1000)
        >
            <crate::barrat_components::Barrat barrat_signal=barrat card_sorting_signal=card_sorting/>
        </AnimatedShow>
        <AnimatedShow
            when=card_sorting
            show_class = "fade-in-1000"
            hide_class = "fade-out-1000"
            hide_delay = Duration::from_millis(1000)
        >
            <crate::questions_components::Questions questions_signal=questions barrat_signal=barrat/>
        </AnimatedShow>
        <AnimatedShow
            when=card_game
            show_class = "fade-in-1000"
            hide_class = "fade-out-1000"
            hide_delay = Duration::from_millis(1000)
        >
            <crate::questions_components::Questions questions_signal=questions barrat_signal=barrat/>
        </AnimatedShow>
        <AnimatedShow
            when=card_gameqs
            show_class = "fade-in-1000"
            hide_class = "fade-out-1000"
            hide_delay = Duration::from_millis(1000)
        >
            <crate::questions_components::Questions questions_signal=questions barrat_signal=barrat/>
        </AnimatedShow>
        <AnimatedShow
            when=thanks
            show_class = "fade-in-1000"
            hide_class = "fade-out-1000"
            hide_delay = Duration::from_millis(1000)
        >
            <crate::questions_components::Questions questions_signal=questions barrat_signal=barrat/>
        </AnimatedShow>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        //<Stylesheet id="leptos" href="/pkg/psicotesis.css"/>

        // sets the document title
        <Title text="Evaluación neuropsicológica"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=Tests/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
