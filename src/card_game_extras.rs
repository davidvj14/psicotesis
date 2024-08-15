use chrono::Utc;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameState {
    pub ttf: i64,
    pub time: i64,
    pub choices: [i64; 5],
    pub score: i64,
    pub qs: [u8; 3],
}

pub fn inc_choice(state: RwSignal<GameState>, n: i64) {
    let mut s = state.get();
    s.choices[n as usize - 1] += 1;
    state.set(GameState {
        ttf: s.ttf,
        time: s.time,
        choices: s.choices,
        score: s.score,
        qs: s.qs,
    })
}

pub fn inc_time(state: RwSignal<GameState>, time: i64) {
    let mut s = state.get();
    s.time += time;
    state.set(GameState {
        ttf: s.ttf,
        time: s.time,
        choices: s.choices,
        score: s.score,
        qs: s.qs,
    })
}

pub fn set_ttf(state: RwSignal<GameState>, time: i64) {
    let s = state.get();
    state.set(GameState {
        ttf: time,
        time: s.time,
        choices: s.choices,
        score: s.score,
        qs: s.qs,
    })
}

pub fn update_score(state: RwSignal<GameState>, card: i64, punish: i64) {
    let mut s = state.get();
    s.score = s.score + card + punish;
    state.set(GameState {
        ttf: s.ttf,
        time: s.time,
        choices: s.choices,
        score: s.score,
        qs: s.qs,
    })
}

pub fn set_q_val(state: RwSignal<GameState>, n: usize, val: u8) {
    let mut s = state.get();
    s.qs[n] = val;
    state.set(GameState {
        ttf: s.ttf,
        time: s.time,
        choices: s.choices,
        score: s.score,
        qs: s.qs,
    })
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
            qs: [0; 3],
        }
    }
}

#[derive(Clone)]
pub struct UiSignals {
    pub game: RwSignal<bool>,
    pub show_end: RwSignal<bool>,
    pub reading: RwSignal<bool>,
    pub times_over: RwSignal<bool>,
    pub done: RwSignal<bool>,
    pub questions: RwSignal<bool>,
}

impl UiSignals {
    pub fn new(game: RwSignal<bool>, show_end: RwSignal<bool>) -> Self {
        Self {
            game,
            show_end,
            reading: create_rw_signal(true),
            times_over: create_rw_signal(false),
            done: create_rw_signal(false),
            questions: create_rw_signal(false),
        }
    }
}

#[allow(unused)]
#[derive(Clone)]
pub struct GameSignals {
    pub game_state: RwSignal<GameState>,
    pub ui_state: UiSignals,
    pub reading_signal: RwSignal<bool>,
    pub state: RwSignal<GameState>,
    pub times_over: RwSignal<bool>,
    pub timer_signal: RwSignal<i64>,
    pub card_signals: [RwSignal<bool>; 5],
    pub indices: [RwSignal<i64>; 5],
    pub stack_signals: [RwSignal<usize>; 5],
}

impl GameSignals {
    pub fn new(game: RwSignal<bool>, show_end: RwSignal<bool>) -> Self {
        Self {
            game_state: create_rw_signal(GameState::new()),
            ui_state: UiSignals::new(game, show_end),
            reading_signal: create_rw_signal(true),
            state: create_rw_signal(GameState::new()),
            times_over: create_rw_signal(false),
            timer_signal: create_rw_signal(0i64),
            card_signals: [create_rw_signal(false); 5],
            indices: [create_rw_signal(0i64); 5],
            stack_signals: [create_rw_signal(0); 5],
        }
    }
}

pub fn any_card_active(gs: RwSignal<GameSignals>) -> bool {
    let card_signals = gs.get().card_signals;
    for cs in card_signals {
        if cs.get() {
            return true;
        }
    }
    false
}

pub fn start_game(game_signals: RwSignal<GameSignals>) {
    let gs = game_signals.get();
    gs.ui_state.reading.set(false);
    gs.timer_signal.set(Utc::now().timestamp_millis());
    game_signals.set(gs);
}

pub fn get_state(game_signals: RwSignal<GameSignals>) -> RwSignal<GameState> {
    game_signals.get().game_state
}

pub fn get_timer(game_signals: RwSignal<GameSignals>) -> i64 {
    game_signals.get().timer_signal.get()
}

pub fn set_timer_now(game_signals: RwSignal<GameSignals>) {
    let t = Utc::now().timestamp_millis();
    game_signals.get().timer_signal.set(t);
}

pub fn is_first(game_signals: RwSignal<GameSignals>) -> bool {
    game_signals.get().game_state.get().ttf == 0
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
    pub value: i64,
}

pub static STACK1: [Card; 18] = [
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -2 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -2 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
];

pub static STACK2: [Card; 18] = [
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -3 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -3 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -3 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -3 },
    Card { value: 0 },
    Card { value: 0 },
];

pub static STACK3: [Card; 18] = [
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -5 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -5 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -5 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -5 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -5 },
    Card { value: 0 },
    Card { value: 0 },
    Card { value: -5 },
];

pub static STACK4: [Card; 18] = [
    Card { value: 0 },
    Card { value: -8 },
    Card { value: 0 },
    Card { value: -8 },
    Card { value: 0 },
    Card { value: -8 },
    Card { value: 0 },
    Card { value: -8 },
    Card { value: 0 },
    Card { value: -8 },
    Card { value: 0 },
    Card { value: -8 },
    Card { value: 0 },
    Card { value: -8 },
    Card { value: 0 },
    Card { value: -8 },
    Card { value: 0 },
    Card { value: -8 },
];

pub static STACK5: [Card; 18] = [
    Card { value: 0 },
    Card { value: -12 },
    Card { value: 0 },
    Card { value: -12 },
    Card { value: -12 },
    Card { value: 0 },
    Card { value: -12 },
    Card { value: 0 },
    Card { value: -12 },
    Card { value: -12 },
    Card { value: 0 },
    Card { value: -12 },
    Card { value: 0 },
    Card { value: -12 },
    Card { value: -12 },
    Card { value: 0 },
    Card { value: -12 },
    Card { value: 0 },
];
