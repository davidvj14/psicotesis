use crate::card_game_extras::*;
use crate::app::Stage;
use chrono::prelude::*;
use leptos::*;
use leptos_meta::*;

const GAME_DURATION: u64 = 300;

#[component]
pub fn DirectCardGame() -> impl IntoView {
    view! {
        <CardGame stage=create_rw_signal(Stage::CardGame)/>
    }
}

#[component]
pub fn CardGame(stage: RwSignal<Stage>) -> impl IntoView {
    let gs = create_rw_signal(GameSignals::new());
    view! {
        <Stylesheet href="card_game.css"/>
        <body>
            <div class="container">
                <Show when=move || gs.get().ui_state.reading_instructions.get() >
                    <Instructions gs=gs/>
                </Show>
                <Show when=move || gs.get().ui_state.game.get() >
                    <div class="score-container">
                        <div class="score">
                            {move || get_score(gs)}
                        </div>
                    </div>
                    <div class="score-bar-container">
                        <div class="score-bar">
                            <ScoreBar gs=gs/>
                        </div>
                    </div>
                    <div class="cards-row" id="upperRow">
                        <UpperCards gs=gs/>
                    </div>
                    <div class="cards-row" id="lowerRow">
                        <LowerCards gs=gs/>
                    </div>
                </Show>
                <Show when=move || gs.get().ui_state.show_end.get() >
                    <EndOfGameText gs=gs/>
                </Show>
                <Show when=move || gs.get().ui_state.questions.get() >
                    <Questions gs=gs stage=stage/>
                </Show>
            </div>
        </body>
    }
}

#[component]
fn ScoreBar(gs: RwSignal<GameSignals>) -> impl IntoView {
    view! {
        <div class="score-bar">
            <div class="score-bar-inner" id="scoreBar"
                style:width=move || {
                    format!("{}%", (get_score(gs) as f64 / 190.0) * 100.0)
                }
                style:left=move || {
                    if get_score(gs) >= 0 {
                        format!("50%")
                    } else {
                        format!("")
                    }
                }
                style:right=move || {
                    if get_score(gs) < 0 {
                        format!("50%")
                    } else {
                        format!("")
                    }
                }
                style:background-color=move || {
                    if get_score(gs) > 0 {"green"} else {"red"}
                }
                >
            </div>
        </div>
    }
}

#[component]
fn LowerCards(gs: RwSignal<GameSignals>) -> impl IntoView {
    view! {
        {
            (1..=5).map(|x| view! {
                <LowerCard gs={gs} n={x} />
            }).collect::<Vec<_>>()
        }
    }
}

#[component]
fn UpperCards(gs: RwSignal<GameSignals>) -> impl IntoView {
    view !{
        {
            (0..=4).map(|x| view! {
                <UpperCard signal={gs.get().card_signals[x]} stack={&STACKS[x]} index={gs.get().stack_indices[x]}/>
            }).collect::<Vec<_>>()
        }
    }
}

#[component]
fn UpperCard(
    signal: RwSignal<bool>,
    stack: &'static [Card; 18],
    index: RwSignal<usize>,
) -> impl IntoView {
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
fn LowerCard(
    gs: RwSignal<GameSignals>,
    n: i64,
) -> impl IntoView {
    view! {
        <div class="card" style:background-color= move || {
            if gs.get().card_signals[n as usize - 1].get() {
                format!("#999999")
            } else {
                format!("#ffffff")
            }
        }
            on:click = move |_| {
                card_click_handler(gs, n);
            }>{n}</div>
    }
}

fn card_click_handler(gs: RwSignal<GameSignals>, n: i64) {
    if any_card_active(gs) {
        return;
    }

    let time = Utc::now().timestamp_millis() - get_timer(gs);
    set_timer_now(gs);

    if is_first(gs) {
        set_ttf(gs, time);
    }

    inc_choice(gs, n);
    inc_time(gs, time);

    let index_signal = gs.get().stack_indices[n as usize - 1];
    if index_signal.get() >= 18 {
        index_signal.set(0);
    }

    let punish_card_value = STACKS[n as usize - 1][index_signal.get()].value;
    update_score(gs, n, punish_card_value);
    index_signal.set(index_signal.get() + 1);
    gs.get().card_signals[n as usize - 1].set(true);

    set_timeout(move || gs.get().card_signals[n as usize - 1].set(false), std::time::Duration::from_millis(800));

    if reached_max(gs) {
        set_timeout(
            move || {
                gs.get().ui_state.game.set(false);
                gs.get().ui_state.show_end.set(true);
            },
            std::time::Duration::from_millis(800)
        );
    }
}

#[component]
fn Instructions(
    gs: RwSignal<GameSignals>,
) -> impl IntoView {
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
                    start_game(gs);
                    set_timeout(
                        move|| {
                            if gs.get().ui_state.game.get() {
                                gs.get().ui_state.game.set(false);
                                gs.get().ui_state.show_end.set(true);
                            }
                        },
                        std::time::Duration::new(GAME_DURATION, 0)
                    );
                }>
                "Comenzar"
            </button>
        </div>
    }
}

#[component]
fn EndOfGameText(gs: RwSignal<GameSignals>) -> impl IntoView {
    view! {
        <div class="instructions">
            <label>
                "Has concluido esta parte, a continuación se te pedirá responder unas\
                preguntas sencillas relacionadas al juego anterior."
            </label>
            <button class="instructions_ok" on:click=move |_| {
                gs.get().ui_state.show_end.set(false);
                gs.get().ui_state.questions.set(true);
            }
            >
                "Continuar"
            </button>
        </div>
    }
}

#[component]
fn Questions(gs: RwSignal<GameSignals>, stage: RwSignal<Stage>) -> impl IntoView {
    view! {
        <div class="question">
            <label>"¿Qué grupo de cartas eran las que más te daban puntos?"</label>
            <select
                on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_q_val(gs, 0, new_value.parse::<i64>().unwrap());
                }
            >
                <option value=1>"1"</option>
                <option value=2>"2"</option>
                <option value=3>"3"</option>
                <option value=4>"4"</option>
                <option value=5>"5"</option>
            </select>
        </div>
        <div class="question">
            <label>"¿Con qué grupo de cartas te quedabas con menos puntos?"</label>
            <select
                on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_q_val(gs, 1, new_value.parse::<i64>().unwrap());
                }
            >
                <option value=1>"1"</option>
                <option value=2>"2"</option>
                <option value=3>"3"</option>
                <option value=4>"4"</option>
                <option value=5>"5"</option>
            </select>
        </div>
        <div class="question">
            <label>"¿Qué grupo de cartas te quitaba puntos con más frecuencia?"</label>
            <select
                on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_q_val(gs, 2, new_value.parse::<i64>().unwrap());
                }
            >
                <option value=1>"1"</option>
                <option value=2>"2"</option>
                <option value=3>"3"</option>
                <option value=4>"4"</option>
                <option value=5>"5"</option>
            </select>
        </div>
        <br/>
        <button on:click=move |_| {
            spawn_local(async move {
                let _ = crate::card_game::process_card_game(gs.get().game_state).await;
                stage.set(Stage::Thanks);
            })
        }>
            "Continuar"
        </button>
    }
}
