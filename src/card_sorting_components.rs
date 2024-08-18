use crate::{app::Stage, card_sorting_extras::*};
use chrono::prelude::*;
use leptos::*;
use leptos_meta::*;

#[derive(Clone)]
struct State {
    current_card: RwSignal<Card>,
    score: RwSignal<u64>,
    answers: RwSignal<Vec<Answer>>,
    current_criterion: RwSignal<Criterion>,
    current_index: RwSignal<usize>,
    status: RwSignal<GameStatus>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameStatus {
    InProgress,
    Done,
    TimeOver,
}

impl State {
    pub fn new(criterion: Criterion) -> Self {
        Self {
            current_card: create_rw_signal(CARDS[0].clone()),
            score: create_rw_signal(0),
            answers: create_rw_signal(Vec::new()),
            current_criterion: create_rw_signal(criterion),
            current_index: create_rw_signal(0),
            status: create_rw_signal(GameStatus::InProgress),
        }
    }
}

#[component]
pub fn DirectCardSorting() -> impl IntoView {
    view! {
        <CardSorting stage=create_rw_signal(Stage::CardSorting)/>
    }
}

#[component]
pub fn CardSorting(stage: RwSignal<Stage>) -> impl IntoView {
    let reading_signal = create_rw_signal(true);
    let incorrect_signal = create_rw_signal(false);
    let timer_signal = create_rw_signal(0i64);
    let state = create_rw_signal(State::new(CRITERIA[0]));

    view! {
        <Stylesheet href="card_sorting.css"/>
        <div class="container">
            <Show when=move || reading_signal.get()>
                <Instructions reading_signal=reading_signal times_over=state.get().status timer_signal=timer_signal/>
            </Show>
            <Finished state=state stage=stage/>
            <Show when=move || {
                let status = state.get().status.get();
                return !reading_signal.get() && (status != GameStatus::Done && status != GameStatus::TimeOver)
            } >
                <CriteriaCards/>
                <br/>
                <div id="card-area">
                    <Incorrect incorrect_signal=incorrect_signal/>
                    <SortingAreas state=state incorrect_signal=incorrect_signal timer_signal=timer_signal/>
                </div>
                <div id="deck-area">
                    <div class="card" id="deck-card" draggable="true" on:dragstart=move |_| {()}>
                        <img style="overflow: hidden" src=move||state.get().current_card.get().image/>
                    </div>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn Incorrect(incorrect_signal: RwSignal<bool>) -> impl IntoView {
    view! {
        <Show when=move || incorrect_signal.get()>
            <div id="message" class="container">"Incorrecto"</div>
        </Show>
    }
}

#[component]
fn Finished(
    state: RwSignal<State>,
    stage: RwSignal<Stage>
) -> impl IntoView {
    let status = move || state.get().status;
    view! {
        <Show when=move || status().get() == GameStatus::TimeOver || status().get() == GameStatus::Done>
            <div>
                <label>
                "Has concluido la prueba"
                </label>
            </div>
            <button on:click=move |_| {
                let result = crate::card_sorting_extras::TestResult::eval(&state.get().answers.get());
                spawn_local(async move {
                    let _ = crate::card_sorting::process_card_sorting(result).await;
                });
                stage.set(Stage::CardGame);
            }>
                "Siguiente"
            </button>
        </Show>
    }
}

#[component]
fn CriteriaCards() -> impl IntoView {
    view! {
        <div id="criteria-cards">
            <CriterionCard card=CRITERION_CARDS[0].clone()/>
            <CriterionCard card=CRITERION_CARDS[1].clone()/>
            <CriterionCard card=CRITERION_CARDS[2].clone()/>
            <CriterionCard card=CRITERION_CARDS[3].clone()/>
        </div>
    }
}

#[component]
fn CriterionCard(card: Card) -> impl IntoView {
    view! {
        <div class="sorting-area">
            <img style="overflow: hidden" src=move||card.image/>
        </div>
    }
}

#[component]
fn SortingAreas(
    state: RwSignal<State>,
    incorrect_signal: RwSignal<bool>,
    timer_signal: RwSignal<i64>,
) -> impl IntoView {
    view! {
        <div id="card-area">
            <SortingArea
                a_id = 0
                state=state
                timer_signal=timer_signal
                incorrect_signal=incorrect_signal
            /><SortingArea
                a_id = 1
                state=state
                timer_signal=timer_signal
                incorrect_signal=incorrect_signal
            /><SortingArea
                a_id = 2
                state=state
                timer_signal=timer_signal
                incorrect_signal=incorrect_signal
            /><SortingArea
                a_id = 3
                state=state
                timer_signal=timer_signal
                incorrect_signal=incorrect_signal
            />
        </div>

    }
}

#[component]
fn SortingArea(
    a_id: usize,
    state: RwSignal<State>,
    timer_signal: RwSignal<i64>,
    incorrect_signal: RwSignal<bool>,
) -> impl IntoView {
    let area_card = create_rw_signal(None);
    view! {
        <div class="sorting-area" on:drop=move |event| {
                event.prevent_default();
                let time = Utc::now().timestamp_millis() - timer_signal.get();
                let (answers, result) = eval_answer(
                    state.get().current_criterion.get(),
                    a_id,
                    state.get().current_card.get(),
                    state.get().score,
                    state.get().answers,
                    time,
                    );
                if !result {
                    show_incorrect(incorrect_signal);
                }
                let score = state.get().score.get();
                if score % 10 == 0 {
                    state.get().current_criterion.set(CRITERIA[(score / 10) as usize]);
                }
                state.get().answers.set(answers);
                area_card.set(Some(state.get().current_card.get()));
                state.get().current_index.set(state.get().current_index.get() + 1);
                if state.get().current_index.get() == 64 {
                    state.get().status.set(GameStatus::Done);
                }
                let status = state.get().status.get();
                if status == GameStatus::Done || status == GameStatus::TimeOver {
                    return;
                }
                state.get().current_card.set(CARDS[state.get().current_index.get()].clone());
                timer_signal.set(Utc::now().timestamp_millis());
            }
        on:dragover=move |event| (event.prevent_default())>
            <Show when=move || area_card.get().is_some()>
                <img style="overflow:hidden" src=move || area_card.get().unwrap().image/>
            </Show>
        </div>
    }
}

fn show_incorrect(incorrect_signal: RwSignal<bool>) {
    incorrect_signal.set(true);
    set_timeout(
        move || incorrect_signal.set(false),
        std::time::Duration::new(1, 0),
    );
}

#[component]
fn Instructions(
    reading_signal: RwSignal<bool>,
    times_over: RwSignal<GameStatus>,
    timer_signal: RwSignal<i64>,
) -> impl IntoView {
    view! {
        <div id="instructions" class="container">
        "\
            En esta tarea lo que tiene que hacer es tomar cada una de las cartas \
            mostradas y colocarlas sobre una de las zonas designadas según como \
            crea que se relacionan o deban clasificarse. Los criterios de \
            clasificación irán cambiando conforme avance la prueba. Si la carta \
            que colocó es correcta, no sucederá nada, pero si es incorrecta, \
            se le notificará. Entonces tome la siguiente carta y trate de colocarla \
            en el lugar adecuado.\
        "
        <br/>
        <button id="instructions_ok" on:click=move |_| {
                reading_signal.set(false);
                timer_signal.set(Utc::now().timestamp_millis());
                set_timeout(
                    move|| times_over.set(GameStatus::TimeOver),
                    std::time::Duration::new(600, 0)
                );
            }>
            "Comenzar"
        </button>
        </div>
    }
}
