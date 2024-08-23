use leptos::*;
use leptos_meta::Stylesheet;
use crate::extras::read_cookie;

#[component]
pub fn Ending() -> impl IntoView {
    let cookie_value = move || read_cookie("p_id").unwrap_or(String::new());
    view! {
        <Stylesheet href="barrat.css"/>
        <div class="container">
            <p>
                "Has concluido las pruebas digitales.
                Ahora deberás ir con la persona que está
                aplicando la prueba y darle tu número de participante
                para que te aplique una última prueba.
                Tu número de participante es: " {cookie_value}
            </p>
        </div>
    }
}
