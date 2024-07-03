#![allow(unused)]
use crate::card_sorting_extras::*;
use chrono::{prelude::*, TimeDelta};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn CardSorting() -> impl IntoView {
    let reading_signal = create_rw_signal(true);
    let incorrect_signal = create_rw_signal(false);
    let current_card = create_rw_signal(CARDS[0].clone());
    let correct_answers = create_rw_signal(0);
    let mut criteria = vec![
        Criterion::Color,
        Criterion::Shape,
        Criterion::Number,
        Criterion::Shape,
        Criterion::Number,
        Criterion::Color,
    ];
    criteria.reverse();
    let current_criterion = create_rw_signal(criteria.pop().unwrap());
    let current_index = create_rw_signal(0);
    let timer_signal = create_rw_signal(0i64);

    view! {
        <Stylesheet href="card_sorting.css"/>
        <div class="container">
            <Show when=move || reading_signal.get()>
                <Instructions reading_signal=reading_signal timer_signal=timer_signal/>
            </Show>
            <Show when=move || !reading_signal.get()>
                <div id="card-area">
                    <Incorrect incorrect_signal=incorrect_signal/>
                    <SortingArea id="area1" index_signal=current_index card_signal=current_card timer_signal=timer_signal incorrect_signal=incorrect_signal/>
                    <SortingArea id="area2" index_signal=current_index card_signal=current_card timer_signal=timer_signal incorrect_signal=incorrect_signal/>
                    <SortingArea id="area3" index_signal=current_index card_signal=current_card timer_signal=timer_signal incorrect_signal=incorrect_signal/>
                    <SortingArea id="area4" index_signal=current_index card_signal=current_card timer_signal=timer_signal incorrect_signal=incorrect_signal/>
                </div>
                <div id="deck-area">
                    <div class="card" id="deck-card" draggable="true" on:dragstart=move |_| {()}>
                        <img style="overflow: hidden" src=move||current_card.get().image/>
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
fn SortingArea(
    id: &'static str,
    index_signal: RwSignal<usize>,
    card_signal: RwSignal<Card>,
    timer_signal: RwSignal<i64>,
    incorrect_signal: RwSignal<bool>,
) -> impl IntoView {
    let area_image = create_rw_signal(None);
    view! {
        <div class="sorting-area" id=id on:drop=move |event| {
                event.prevent_default();
                logging::log!("{:?} ms", Utc::now().timestamp_millis() - timer_signal.get());
                area_image.set(Some(card_signal.get().image));
                index_signal.set(index_signal.get() + 1);
                card_signal.set(CARDS[index_signal.get()].clone());
                timer_signal.set(Utc::now().timestamp_millis());
                incorrect_signal.set(true);
                set_timeout(
                    move || incorrect_signal.set(false),
                    std::time::Duration::new(1, 0),
                );
            }
        on:dragover=move |event| (event.prevent_default())>
            <Show when=move || area_image.get().is_some()>
                <img style="overflow:hidden" src=move || area_image.get().unwrap()/>
            </Show>
        </div>
    }
}

#[component]
fn Instructions(reading_signal: RwSignal<bool>, timer_signal: RwSignal<i64>) -> impl IntoView {
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
            }>
            "Comenzar"
        </button>
        </div>
    }
}
