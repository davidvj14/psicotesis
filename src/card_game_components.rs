#![allow(unused)]

use leptos::*;
use leptos_meta::*;
use chrono::{prelude::*, Duration};
use crate::card_game_extras::*;

#[component]
pub fn DirectCardGame() -> impl IntoView {
    let gs = create_rw_signal(true);
    let qs = create_rw_signal(false);

    view! {
        <CardGame game_signal=gs gameqs_signal=qs/>
    }
}

#[component]
pub fn CardGame(game_signal: RwSignal<bool>, gameqs_signal: RwSignal<bool>) -> impl IntoView {
    let reading_signal = create_rw_signal(true);
    let state = create_rw_signal(GameState::new());
    let values = create_rw_signal(['\0'; 3]);
    let q_signal = create_rw_signal(false);
    let times_over = create_rw_signal(false);
    let timer_signal = create_rw_signal(0i64);
    let card1_signal = create_rw_signal(false);
    let card2_signal = create_rw_signal(false);
    let card3_signal = create_rw_signal(false);
    let card4_signal = create_rw_signal(false);
    let card5_signal = create_rw_signal(false);
    let stack1_signal = create_rw_signal(0);
    let stack2_signal = create_rw_signal(0);
    let stack3_signal = create_rw_signal(0);
    let stack4_signal = create_rw_signal(0);
    let stack5_signal = create_rw_signal(0);
    view! {
        <Stylesheet href="card_game.css"/>
        <body>
            <div class="container">
                <Show when=move || reading_signal.get()>
                    <Instructions reading_signal=reading_signal times_over=times_over timer_signal=timer_signal/>
                </Show>
                <Show when=move || !reading_signal.get() && !times_over.get() && !reached_max(state)>
                    <div class="score-container">
                        <div class="score">
                            {state.get().score}
                        </div>
                    </div>
                    <div class="score-bar-container">
                        <div class="score-bar">
                            <div class="score-bar-inner" id="scoreBar"
                                style:width=move || {
                                    format!("{}%", (state.get().score.abs() as f64 / 190.0) * 100.0)
                                }
                                style:left=move || {
                                    if state.get().score >= 0 {
                                        format!("50%")
                                    } else {
                                        format!("")
                                    }
                                }
                                style:right=move || {
                                    if state.get().score < 0 {
                                        format!("50%")
                                    } else {
                                        format!("")
                                    }
                                }
                                style:background-color=move || {if state.get().score > 0 {"green"} else {"red"}}
                                >
                            </div>
                        </div>
                    </div>
                    <div class="cards-row" id="upperRow">
                        <UpperCard signal=card1_signal stack=&STACK1 index=stack1_signal/>
                        <UpperCard signal=card2_signal stack=&STACK2 index=stack2_signal/>
                        <UpperCard signal=card3_signal stack=&STACK3 index=stack3_signal/>
                        <UpperCard signal=card4_signal stack=&STACK4 index=stack4_signal/>
                        <UpperCard signal=card5_signal stack=&STACK5 index=stack5_signal/>
                    </div>
                    <div class="cards-row" id="lowerRow">
                        <LowerCard signal=card1_signal n=1 stack=&STACK1
                            index=stack1_signal timer=timer_signal state=state/>
                        <LowerCard signal=card2_signal n=2 stack=&STACK2
                            index=stack2_signal timer=timer_signal state=state/>
                        <LowerCard signal=card3_signal n=3 stack=&STACK3
                            index=stack3_signal timer=timer_signal state=state/>
                        <LowerCard signal=card4_signal n=4 stack=&STACK4
                            index=stack4_signal timer=timer_signal state=state/>
                        <LowerCard signal=card5_signal n=5 stack=&STACK5
                            index=stack5_signal timer=timer_signal state=state/>
                    </div>
                </Show>
                <Show when=move || {(times_over.get() || reached_max(state)) && !q_signal.get() }>
                    <EndOfGameText q_signal=q_signal/>
                </Show>
                <Show when=move || q_signal.get()>
                    <Questions values=values/>
                </Show>
            </div>
        </body>
    }    
}

#[component]
fn UpperCard(signal: RwSignal<bool>, stack: &'static [Card; 18], index: RwSignal<usize>) -> impl IntoView {
    view! {
        <div class:backside=move || !signal.get()
             class:reveal-neutral=move || {signal.get() && stack[index.get() - 1].value == 0 }
             class:reveal-punish=move || {signal.get() && stack[index.get() - 1].value != 0 }>
            <Show when=move || signal.get()>
                   <label>{stack[index.get() - 1].value}</label>
            </Show>
        </div>
    }
}

#[component]
fn LowerCard(signal: RwSignal<bool>, n: i64, stack: &'static [Card; 18], index: RwSignal<usize>,
    timer: RwSignal<i64>, state: RwSignal<GameState>) -> impl IntoView {
    view! {
        <div class="card" style:background-color= move || {
            if signal.get() {
                format!("#999999")
            } else {
                format!("#ffffff")
            }
        }
            on:click = move |_| {
            if signal.get() {
                return;
            }
            let time = Utc::now().timestamp_millis() - timer.get();
            if state.get().ttf == 0 {
                set_ttf(state, time);
            }
            inc_choice(state, n);
            inc_time(state, time);
            if index.get() >= 18 {
                index.set(0);
            }
            let p_val = stack[index.get()].value;
            update_score(state, n, p_val);
            timer.set(Utc::now().timestamp_millis());
            index.set(index.get() + 1);
            logging::log!("{:?}", state.get());
            signal.set(true);
            set_timeout(move || signal.set(false), std::time::Duration::from_millis(800));
        }>{n}</div>
    }
}

#[component]
fn Instructions(reading_signal: RwSignal<bool>, times_over: RwSignal<bool>,
    timer_signal: RwSignal<i64>) -> impl IntoView {
    view! {
        <div id="instructions">
            "El objetivo de esta tarea es lograr la mayor cantidad posible de puntos. Para lograr esto, \
            podrás escoger cartas con valor desde 1 hasta 5 puntos, las cuales estarán ubicadas en la \
            parte inferior de la pantalla, estas las puedes seleccionar en el orden que prefieras y las \
            veces que quieras."
            <br/>
            <br/>
            "En la parte superior de la pantalla se encontrará otros 5 grupos de cartas que estarán \
            ocultas, estas estarán alineadas a los 5 grupos inferiores. Estas pueden o no contener \
            castigos. Cada vez que selecciones una carta de la parte inferior, se revelará la carta \
            del grupo oculto que le corresponde." 
            <br/>
            <br/>
            "Si la carta oculta revela un “0” conservaras los puntos, mientras que si se muestra un \
            “-2” perderás esa cantidad de puntos."
            <br/>
            <br/>
            <button id="instructions_ok" on:click=move |_| {
                    reading_signal.set(false);
                    timer_signal.set(Utc::now().timestamp_millis());
                    set_timeout(
                        move|| times_over.set(true),
                        std::time::Duration::new(300, 0)
                    );
                }>
                "Comenzar"
            </button>
        </div>
    }
}

#[component]
fn EndOfGameText(q_signal: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="instructions">
            <label>
                "Has concluido esta parte, a continuación se te pedirá responder unas\
                preguntas sencillas relacionadas al juego anterior."
            </label>
            <button class="instructions_ok" on:click=move |_| {
                q_signal.set(true);
            }
            >
                "Continuar"
            </button>
        </div>
    }
}

#[component]
fn Questions(values: RwSignal<[char; 3]>) -> impl IntoView {
    view! {
        <div class="question">
            <label>"¿Qué grupo de cartas eran las que más te daban puntos?"</label>
            <select
                on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    let mut vs = values.get();
                    vs[0] = new_value.chars().next().unwrap();
                }
            >
                <option value='1'>"1"</option>
                <option value='2'>"2"</option>
                <option value='3'>"3"</option>
                <option value='4'>"4"</option>
                <option value='5'>"5"</option>
            </select>
        </div>
        <div class="question">
            <label>"¿Con qué grupo de cartas te quedabas con menos puntos?"</label>
            <select
                on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    let mut vs = values.get();
                    vs[1] = new_value.chars().next().unwrap();
                }
            >
                <option value='1'>"1"</option>
                <option value='2'>"2"</option>
                <option value='3'>"3"</option>
                <option value='4'>"4"</option>
                <option value='5'>"5"</option>
            </select>
        </div>
        <div class="question">
            <label>"¿Qué grupo de cartas te quitaba puntos con más frecuencia?"</label>
            <select
                on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    let mut vs = values.get();
                    vs[2] = new_value.chars().next().unwrap();
                }
            >
                <option value='1'>"1"</option>
                <option value='2'>"2"</option>
                <option value='3'>"3"</option>
                <option value='4'>"4"</option>
                <option value='5'>"5"</option>
            </select>
        </div>
    }
}
