use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Card {
    id: usize,
    image: String,
    correct_area: String,
}

#[component]
pub fn CardSorting() -> impl IntoView {
    let cards = vec![
        Card {
            id: 1,
            image: "card1.jpg".into(),
            correct_area: "sorting-area-1".into(),
        },
        Card {
            id: 2,
            image: "card2.jpg".into(),
            correct_area: "sorting-area-2".into(),
        },
        // Add all 30 cards here
        Card {
            id: 30,
            image: "card30.jpg".into(),
            correct_area: "sorting-area-4".into(),
        },
    ];

    let current_card = create_signal(cards[0].clone());
    let current_index = create_signal(0);

    view! {
        <Stylesheet href="card_sorting.css"/>
        <div class="container">
            <h1>"Wisconsin Card Sorting Test"</h1>
            <div id="instructions">
                <p>"Sort the cards based on the rule. Drag a card from the deck to one of the sorting areas."</p>
            </div>
            <div id="message" class="hidden">"Incorrect"</div>
            <div id="card-area">
                <div class="sorting-area" id="sorting-area-1" on:drop=move |event| {()} on:dragover=move |_| ()></div>
                <div class="sorting-area" id="sorting-area-2" on:drop=move |event| {()} on:dragover=move |_| ()></div>
                <div class="sorting-area" id="sorting-area-3" on:drop=move |event| {()} on:dragover=move |_| ()></div>
                <div class="sorting-area" id="sorting-area-4" on:drop=move |event| {()} on:dragover=move |_| ()></div>
            </div>
            <div id="deck-area">
                <div class="card" id="deck-card" draggable="true" on:dragstart=move |event| {()}></div>
            </div>
        </div>
    }
}
