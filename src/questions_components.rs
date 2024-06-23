use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
fn AgeQ() -> impl IntoView {
    view! {
        <div class="question">
            <label>"Edad"</label><br/><br/>
            <input type="number" id="age" name="participant[age]" required/><br/><br/>
        </div>
    }
}

#[component]
fn SexQ() -> impl IntoView {
    view! {
        <div class="question">
            <label>"Sexo"<br/><br/></label>
            <label>"Masculino"</label>
            <input type="radio" id="sex_M" name="participant[sex]" value=0 required/><br/>

            <label> "Femenino" </label>
            <input type="radio" id="sex_F" name="participant[sex]" value=1 required/><br/><br/>
        </div>
    }
}

#[component]
fn MajorQ() -> impl IntoView {
    view! {
        <div class="question">
            <label>"Carrera"<br/><br/></label>
            <input type="text" id="major" name="participant[major]" required/><br/>
        </div>
    }
}

#[component]
fn AlcoholQ() -> impl IntoView {
    let (alcohol, set_alcohol) = create_signal(false);
    view! {
        <div class="question">
            <label>"¿Consumes alcohol?"</label><br/>
            <input type="radio" id="alcohol_yes" name="participant[alcohol]" value=1
                on:click=move |_| {set_alcohol.update(|a| *a = true);}/>
            <label for="alcohol_yes">Sí</label><br/>
            <input type="radio" id="alcohol_no" name="participant[alcohol]" value=0
                on:click=move |_| {set_alcohol.update(|a| *a = false);}/>
            <label for="alcohol_no">No</label><br/><br/>

            <Show
                when= move || alcohol.get()
                fallback= || view!{ "" }
            >
                <AlcoholFrequencyQ/>
            </Show>
        </div>
    }
}

#[component]
fn AlcoholFrequencyQ() -> impl IntoView {
    view! {
        <div id="alcohol_frequency">
            <label>"¿Con qué frecuencia consumes? (Sin importar la cantidad)"</label><br/>
            <select id="alcohol" name="participant[alcohol_freq]">
                <option disabled selected value>"Seleccionar frecuencia"</option>
                <option value=0>"Todos los días"</option>
                <option value=1>"Una vez a la semana"</option>
                <option value=2>"Cada dos semanas"</option>
                <option value=4>"Una vez al mes"</option>
                <option value=8>"De manera esporádica"</option>
            </select><br/>
        </div>
    }
}

#[component]
fn DrugsQ() -> impl IntoView {
    let (drugs, set_drugs) = create_signal(false);
    view! {
        <div class="question">
            <label>"¿Consumes drogas?"</label><br/>
            <input type="radio" id="drugs_yes" name="participant[drugs]" value=1
                on:click=move |_| {set_drugs.update(|a| *a = true);}/>
            <label f:or="drugs_yes">Sí</label><br/>
            <input type="radio" id="drugs_no" name="participant[drugs]" value=0
                on:click=move |_| {set_drugs.update(|a| *a = false);}/>
            <label for="drugs_no">No</label><br/><br/>

            <Show
                when= move || drugs.get()
                fallback= || view!{ "" }
            >
                <DrugsFrequencyQ/>
            </Show>
        </div>
    }
}

#[component]
fn DrugsFrequencyQ() -> impl IntoView {
    view! {
        <div id="drugs_frequency">
            <label>"¿Con qué frecuencia consumes? (Sin importar la cantidad)"</label><br/>
            <select id="drugs" name="participant[drugs_freq]">
                <option disabled selected value>"Seleccionar frecuencia"</option>
                <option value=0>"Todos los días"</option>
                <option value=1>"Una vez a la semana"</option>
                <option value=2>"Cada dos semanas"</option>
                <option value=4>"Una vez al mes"</option>
                <option value=8>"De manera esporádica"</option>
            </select><br/>
        </div>
    }
}

#[component]
fn DisorderQ() -> impl IntoView {
    let (disorder, set_disorder) = create_signal(false);
    view! {
        <div class="question">
            <label>
                "¿Tienes algún diagnóstico psiquiátrico o neurológico?"<br/>
            </label>
            <small>"Dicho diagnóstico debe de haber sido designado por un profesional de la salud,\
                    puede ser diagnóstico de ansiedad, depresión, bipolaridad tipo I o II, TDA-H,\
                    autismo, alguna enfermedad neurodegenerativa, etc.\
                    De lo contrario poner 'ninguna.'"</small>
            <input type="radio" id="disorder_yes" name="participant[disorder]" value=1
                on:click=move |_| {set_disorder.update(|a| *a = true);}/>
            <label for="disorder_yes">Sí</label><br/>
            <input type="radio" id="disorder_no" name="participant[disorder]" value=0
                on:click=move |_| {set_disorder.update(|a| *a = false);}/>
            <label for="disorder_no">No</label><br/>

            <Show
                when= move || disorder.get()
                fallback= || view!{ "" }
            >
                "¿Cuál es tu diagnóstico?"<br/>
                <input type="text" id="disorder_input" name="participant[disorder_input]"/><br/><br/>
            </Show>
        </div>
    }
}

#[component]
fn InjuryQ() -> impl IntoView {
    let (injury, set_injury) = create_signal(false);
    view! {
        <div class="question">
            <label>
                "¿Has presentado algún golpe en la cabeza importante?"<br/>
            </label>
            <small>"Por el cual te hayan hecho una tomografía y que haya generado algún traumatismo en el cerebro"</small>
            <input type="radio" id="injury_yes" name="participant[injury]" value=1
                on:click=move |_| {set_injury.update(|a| *a = true);}/>
            <label for="injury_yes">Sí</label><br/>
            <input type="radio" id="injury_no" name="participant[injury]" value=0
                on:click=move |_| {set_injury.update(|a| *a = false);}/>
            <label for="injury_no">No</label><br/><br/>

            <Show
                when= move || injury.get()
                fallback= || view!{ "" }
            >
                <label>"¿En dónde se ubicó el golpe?"</label><br/>
                <input type="text" id="injury_location" name="participant[injury_location]"/><br/><br/>

                <label>"¿Fue tratado el traumatismo?"</label><br/>
                <input type="radio" id="treated_yes" name="participant[treated]" value=1/>
                <label for="treated_yes">Sí</label><br/>
                <input type="radio" id="treated_no" name="participant[treated]" value=0/>
                <label for="treated_no">No</label><br/>
            </Show>
        </div>
    }
}

#[component]
fn AbuseQ() -> impl IntoView {
    view! {
        <div class="question">
            <label>"¿En tu vida viviste algún tipo de abuso?"</label><br/><br/>
            <input type="checkbox" id="abuse_psychological" name="abuse[psychological]" value=1/>
            <label for="abuse_psychological">Psicológico</label><br/>
            <input type="checkbox" id="abuse_physical" name="abuse[physical]" value=2/>
            <label for="abuse_physical">Violencia Física</label><br/>
            <input type="checkbox" id="abuse_sexual" name="abuse[sexual]" value=4/>
            <label for="abuse_sexual">Abuso sexual</label><br/>
            <input type="checkbox" id="abuse_bullying" name="abuse[bullying]" value=8/>
            <label for="abuse_sexual">Abuso escolar o bullying</label><br/>
            <input type="checkbox" id="abuse_finantial" name="abuse[finantial]" value=16/>
            <label for="abuse_sexual">Abuso financiero</label><br/>
            <input type="checkbox" id="abuse_none" name="abuse[_none]" value=0/>
            <label for="abuse_sexual">Ninguno</label><br/>
            <input type="checkbox" id="abuse_other" name="abuse[other]" value=32/>
            <label for="abuse_other">Otro:</label>
            <input type="text" id="abuse_other_text" name="abuse_other_text"/>
        </div>
    }
}

#[component]
fn ShortageQ() -> impl IntoView {
    view! {
        <div class="question">
            <label for="shortage">
                "¿En tu vida viviste carencia económica, social o emocional?"
            </label><br/><br/>
            <input type="checkbox" id="shortage_economic" name="shortage[economic]" value=1/>
            <label for="abuse_psychological">Económica</label><br/>
            <input type="checkbox" id="shortage_social" name="shortage[social]" value=2/>
            <label for="abuse_physical">Social</label><br/>
            <input type="checkbox" id="shortage_emotional" name="shortage[emotional]" value=4/>
            <label for="abuse_sexual">Emocional</label><br/>
            <input type="checkbox" id="shortage_none" name="shortage[_none]" value=0/>
            <label for="abuse_sexual">Ninguna</label><br/>
        </div>
    }
}

#[component]
fn LossQ() -> impl IntoView {
    view! {
        <div class="question">
            <label for="loss">
                "¿Has vivido alguna pérdida importante recientemente?"
            </label><br/>
            <small>Algún familiar, mascota, trabajo, etc.</small>
            <input type="radio" id="loss_yes" name="participant[loss]" value=1/>
            <label for="loss_yes">Sí</label><br/>
            <input type="radio" id="loss_no" name="participant[loss]" value=0/>
            <label for="loss_no">No</label><br/><br/>
        </div>
    }
}

#[component]
pub fn Questions(questions_signal: RwSignal<bool>, barrat_signal: RwSignal<bool>) -> impl IntoView {
    provide_meta_context();
    let pqs = create_server_action::<crate::questions::ProcessQuestions>();

    view! {
        <Stylesheet href="questions.css"/>
        <div class="container">
            <h1>Evaluación neuropsicológica</h1>
            <h3>Hola, muchas gracias por tomarte el tiempo para participar, por favor contesta con sinceridad, se te asignará un número de participante por lo que tus respuestas serán anónimas.</h3>
            <ActionForm action=pqs on:submit=move |_| {questions_signal.set(false); barrat_signal.set(true)}>
                <AgeQ/>
                <SexQ/>
                <MajorQ/>
                <AlcoholQ/>
                <DrugsQ/>
                <DisorderQ/>
                <InjuryQ/>
                <AbuseQ/>
                <ShortageQ/>
                <LossQ/>
                <input type="submit" value="Siguiente"/>
            </ActionForm>
        </div>
    }
}
