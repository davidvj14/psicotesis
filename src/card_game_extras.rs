use chrono::Utc;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameState {
    pub ttf: i64,
    pub time: i64,
    pub choices: [i64; 5],
    pub score: i64,
    pub qs: [i64; 3],
}

pub fn inc_choice(game_signals: RwSignal<GameSignals>, n: i64) {
    let mut state = game_signals.get().game_state;
    state.choices[n as usize - 1] += 1;

    game_signals.set(GameSignals {
        game_state: state,
        ..game_signals.get()
    });
}

pub fn inc_time(game_signals: RwSignal<GameSignals>, time: i64) {
    let mut state = game_signals.get().game_state;
    state.time += time;

    game_signals.set(GameSignals {
        game_state: state,
        ..game_signals.get()
    });
}

pub fn set_ttf(game_signals: RwSignal<GameSignals>, time: i64) {
    let mut state = game_signals.get().game_state;
    state.ttf = time;

    game_signals.set(GameSignals {
        game_state: state,
        ..game_signals.get()
    });
}

pub fn update_score(game_signals: RwSignal<GameSignals>, card: i64, punish: i64) {
    let mut s = game_signals.get().game_state;
    s.score = s.score + card + punish;
    game_signals.set(GameSignals {
        game_state: s,
        ..game_signals.get()
    })
}

pub fn set_q_val(game_signals: RwSignal<GameSignals>, n: usize, val: i64) {
    let mut s = game_signals.get().game_state;
    s.qs[n] = val;
    game_signals.set(GameSignals{
        game_state: s,
        ..game_signals.get()
    });
}

pub fn reached_max(game_signals: RwSignal<GameSignals>) -> bool {
    let c = game_signals.get().game_state.choices;
    (c[0] + c[1] + c[2] + c[3] + c[4]) >= 54
}

impl GameState {
    pub fn new() -> Self {
        Self {
            ttf: 0,
            time: 0,
            choices: [0; 5],
            score: 0,
            qs: [1; 3],
        }
    }
}

#[derive(Clone, Debug)]
pub struct UiSignals {
    pub game: RwSignal<bool>,
    pub show_end: RwSignal<bool>,
    pub reading_instructions: RwSignal<bool>,
    pub questions: RwSignal<bool>,
    pub done: RwSignal<bool>,
}

impl UiSignals {
    pub fn new() -> Self {
        Self {
            game: create_rw_signal(false),
            show_end: create_rw_signal(false),
            reading_instructions: create_rw_signal(true),
            questions: create_rw_signal(false),
            done: create_rw_signal(false),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameSignals {
    pub game_state: GameState,
    pub ui_state: UiSignals,
    pub times_over: RwSignal<bool>,
    pub timer_signal: RwSignal<i64>,
    pub card_signals: [RwSignal<bool>; 5],
    pub stack_indices: [RwSignal<usize>; 5],
}

impl GameSignals {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
            ui_state: UiSignals::new(),
            times_over: create_rw_signal(false),
            timer_signal: create_rw_signal(0i64),
            card_signals: [create_rw_signal(false), create_rw_signal(false),
                create_rw_signal(false), create_rw_signal(false), create_rw_signal(false)],
            stack_indices: [create_rw_signal(0), create_rw_signal(0),
                create_rw_signal(0), create_rw_signal(0), create_rw_signal(0),],
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
    gs.ui_state.reading_instructions.set(false);
    gs.timer_signal.set(Utc::now().timestamp_millis());
    gs.ui_state.game.set(true);
    game_signals.set(gs);
}

pub fn get_state(game_signals: RwSignal<GameSignals>) -> GameState {
    game_signals.get().game_state
}

pub fn get_timer(game_signals: RwSignal<GameSignals>) -> i64 {
    game_signals.get().timer_signal.get()
}

pub fn get_score(game_signals: RwSignal<GameSignals>) -> i64 {
    game_signals.get().game_state.score
}

pub fn set_timer_now(game_signals: RwSignal<GameSignals>) {
    let t = Utc::now().timestamp_millis();
    game_signals.get().timer_signal.set(t);
}

pub fn is_first(game_signals: RwSignal<GameSignals>) -> bool {
    game_signals.get().game_state.ttf == 0
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

pub static STACKS: [&[Card; 18]; 5] = [
    &STACK1,
    &STACK2,
    &STACK3,
    &STACK4,
    &STACK5,
];
