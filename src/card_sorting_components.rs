#![allow(unused)]
use crate::card_sorting_extras::*;
use chrono::{prelude::*, TimeDelta};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[derive(Clone)]
struct State {
    incorrect: RwSignal<bool>,
    current_card: RwSignal<Card>,
    score: RwSignal<u64>,
    answers: RwSignal<Vec<Answer>>,
    current_criterion: RwSignal<Criterion>,
    current_index: RwSignal<usize>,
}

impl State {
    pub fn new(criterion: Criterion) -> Self {
        Self {
            incorrect: create_rw_signal(false),
            current_card: create_rw_signal(CARDS[0].clone()),
            score: create_rw_signal(0),
            answers: create_rw_signal(Vec::new()),
            current_criterion: create_rw_signal(criterion),
            current_index: create_rw_signal(0),
        }
    }
}

#[component]
pub fn CardSorting() -> impl IntoView {
    let reading_signal = create_rw_signal(true);
    let incorrect_signal = create_rw_signal(false);
    let times_over = create_rw_signal(false);
    let timer_signal = create_rw_signal(0i64);
    let state = create_rw_signal(State::new(CRITERIA[0]));

    view! {
        <Stylesheet href="card_sorting.css"/>
        <div class="container">
            <Show when=move || reading_signal.get()>
                <Instructions reading_signal=reading_signal times_over=times_over timer_signal=timer_signal/>
            </Show>
            <Show when=move || times_over.get()>
                <div>
                    "Se acabo el tiempo de la prueba"
                </div>
            </Show>
            <Show when=move || !reading_signal.get() && !times_over.get() >
                <div id="criteria-cards">
                    <CriterionCard card=CRITERION_CARDS[0].clone()/>
                    <CriterionCard card=CRITERION_CARDS[1].clone()/>
                    <CriterionCard card=CRITERION_CARDS[2].clone()/>
                    <CriterionCard card=CRITERION_CARDS[3].clone()/>
                </div>
                <br/>
                <div id="card-area">
                    <Incorrect incorrect_signal=incorrect_signal/>
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
fn CriterionCard(card: Card) -> impl IntoView {
    view! {
        <div class="sorting-area">
            <img style="overflow: hidden" src=move||card.image/>
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
                state.get().current_card.set(CARDS[state.get().current_index.get()].clone());
                logging::log!("{}", state.get().score.get());
                timer_signal.set(Utc::now().timestamp_millis());
            }
        on:dragover=move |event| (event.prevent_default())>
            <Show when=move || area_card.get().is_some()>
                <img style="overflow:hidden" src=move || area_card.get().unwrap().image/>
            </Show>
        </div>
    }
}

fn eval() {

}

fn show_incorrect(incorrect_signal: RwSignal<bool>) {
    incorrect_signal.set(true);
    set_timeout(
        move || incorrect_signal.set(false),
        std::time::Duration::new(1, 0)
    );
}

#[component]
fn Instructions(reading_signal: RwSignal<bool>, times_over: RwSignal<bool>, timer_signal: RwSignal<i64>) -> impl IntoView {
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
                    move|| times_over.set(true),
                    std::time::Duration::new(600, 0)
                );
            }>
            "Comenzar"
        </button>
        </div>
    }
}
