use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameState {
    pub ttf: i64,
    pub time: i64,
    pub choices: [u8; 5],
    pub score: i64,
}

pub fn inc_choice(state: RwSignal<GameState>, n: i64) {
    let mut s = state.get();
    s.choices[n as usize - 1] += 1;
    state.set(GameState {ttf: s.ttf, time: s.time, choices: s.choices, score: s.score})
}

pub fn inc_time(state: RwSignal<GameState>, time: i64) {
    let mut s = state.get();
    s.time += time;
    state.set(GameState {ttf: s.ttf, time: s.time, choices: s.choices, score: s.score})
}

pub fn set_ttf(state: RwSignal<GameState>, time: i64) {
    let s = state.get();
    state.set(GameState {ttf: time, time: s.time, choices: s.choices, score: s.score})
}

pub fn update_score(state: RwSignal<GameState>, card: i64, punish: i64) {
    let mut s = state.get();
    s.score = s.score + card + punish;
    state.set(GameState {ttf: s.ttf, time: s.time, choices: s.choices, score: s.score})
}

pub fn reached_max(state: RwSignal<GameState>) -> bool {
    let c = state.get().choices;
    (c[0] + c[1] + c[2] + c[3] + c[4]) >= 54
}

impl GameState {
    pub fn new() -> Self {
        Self {
            ttf: 0,
            time: 0,
            choices: [0; 5],
            score: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
    pub value: i64,
}

pub static STACK1: [Card; 18] = [
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: -2},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: -2},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
];

pub static STACK2: [Card; 18] = [
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: -3},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: -3},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: -3},
    Card {value: 0},
    Card {value: 0},
    Card {value: 0},
    Card {value: -3},
    Card {value: 0},
    Card {value: 0},
];

pub static STACK3: [Card; 18] = [
    Card {value: 0},
    Card {value: 0},
    Card {value: -5},
    Card {value: 0},
    Card {value: 0},
    Card {value: -5},
    Card {value: 0},
    Card {value: 0},
    Card {value: -5},
    Card {value: 0},
    Card {value: 0},
    Card {value: -5},
    Card {value: 0},
    Card {value: 0},
    Card {value: -5},
    Card {value: 0},
    Card {value: 0},
    Card {value: -5},
];

pub static STACK4: [Card; 18] = [
    Card {value: 0},
    Card {value: -8},
    Card {value: 0},
    Card {value: -8},
    Card {value: 0},
    Card {value: -8},
    Card {value: 0},
    Card {value: -8},
    Card {value: 0},
    Card {value: -8},
    Card {value: 0},
    Card {value: -8},
    Card {value: 0},
    Card {value: -8},
    Card {value: 0},
    Card {value: -8},
    Card {value: 0},
    Card {value: -8},
];

pub static STACK5: [Card; 18] = [
    Card {value: 0},
    Card {value: -12},
    Card {value: 0},
    Card {value: -12},
    Card {value: -12},
    Card {value: 0},
    Card {value: -12},
    Card {value: 0},
    Card {value: -12},
    Card {value: -12},
    Card {value: 0},
    Card {value: -12},
    Card {value: 0},
    Card {value: -12},
    Card {value: -12},
    Card {value: 0},
    Card {value: -12},
    Card {value: 0},
];
