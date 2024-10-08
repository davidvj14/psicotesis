use crate::card_game_components::*;
use crate::card_sorting_components::{CardSorting, DirectCardSorting};
use crate::error_template::{AppError, ErrorTemplate};
use crate::questions_components::Questions;
use crate::barrat_components::{Barrat, DirectBarrat};
use crate::card_game_components::CardGame;
use crate::ending_components::Ending;
use crate::extras::get_stage_from_cookie;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[cfg(feature = "ssr")]
pub mod ssr {
    use dotenvy::dotenv;
    use leptos::ServerFnError;
    use sqlx::{Connection, PgConnection};

    pub async fn db() -> Result<PgConnection, ServerFnError> {
        dotenv().ok();
        println!("{:?}", std::env::var("DATABASE_URL"));
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Ok(PgConnection::connect(&db_url).await?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stage {
    Questions,
    Barrat,
    CardSorting,
    CardGame,
    Ending,
    Void,
}


#[component]
fn Tests() -> impl IntoView {
    let stage = create_rw_signal(Stage::Questions);
    create_effect(move |_| {
        stage.set(get_stage_from_cookie());
    });

    view! {
        <AnimatedShow
            when=MaybeSignal::derive(move || stage.get() == Stage::Questions)
            show_class = "fade-in-1000"
            hide_class = "fade-out-1000"
            hide_delay = Duration::from_millis(1000)
        >
            <Questions stage=stage/>
        </AnimatedShow>
        <AnimatedShow
            when=MaybeSignal::derive(move || stage.get() == Stage::Barrat)
            show_class = "fade-in-1000"
            hide_class = "fade-out-1000"
            hide_delay = Duration::from_millis(1000)
        >
            <Barrat stage=stage/>
        </AnimatedShow>
        <AnimatedShow
            when=MaybeSignal::derive(move || stage.get() == Stage::CardSorting)
            show_class = "fade-in-1000"
            hide_class = "fade-out-1000"
            hide_delay = Duration::from_millis(1000)
        >
            <CardSorting stage=stage/>
        </AnimatedShow>
        <AnimatedShow
            when=MaybeSignal::derive(move || stage.get() == Stage::CardGame)
            show_class = "fade-in-1000"
            hide_class = "fade-out-1000"
            hide_delay = Duration::from_millis(1000)
        >
            <CardGame stage=stage/>
        </AnimatedShow>
        <AnimatedShow
            when=MaybeSignal::derive(move || stage.get() == Stage::Ending)
            show_class = "fade-in-1000"
            hide_class = "fade-out-1000"
            hide_delay = Duration::from_millis(1000)
        >
            <Ending/>
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
                    <Route path="cards" view=DirectCardSorting/>
                    <Route path="barrat" view=DirectBarrat/>
                    <Route path="game" view=DirectCardGame/>
                    <Route path="end" view=Ending/>
                </Routes>
            </main>
        </Router>
    }
}
