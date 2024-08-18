use crate::barrat;
use crate::app::Stage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Barrat(stage: RwSignal<Stage>) -> impl IntoView {
    let action = create_server_action::<barrat::ProcessBarrat>();
    view! {
        <Stylesheet href="barrat.css"/>
        <ActionForm action=action on:submit=move |_| stage.set(Stage::CardSorting)>
        <div class="container">
            <table>
                <thead>
                    <tr>
                        <th></th>
                        <th>"Raramente o nunca"</th>
                        <th>"Ocasionalmente"</th>
                        <th>"A menudo"</th>
                        <th>"Siempre o casi siempre"</th>
                    </tr>
                </thead>
                    <tbody>
                        <tr>
                            <td>"1. Planifico mis tareas con cuidado"</td>
                            <td><input type="radio" name="data[bq1]" value="1" required/></td>
                            <td><input type="radio" name="data[bq1]" value="2"/></td>
                            <td><input type="radio" name="data[bq1]" value="3"/></td>
                            <td><input type="radio" name="data[bq1]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"2. Hago las cosas sin pensarlas"</td>
                            <td><input type="radio" name="data[bq2]" value="1" required/></td>
                            <td><input type="radio" name="data[bq2]" value="2"/></td>
                            <td><input type="radio" name="data[bq2]" value="3"/></td>
                            <td><input type="radio" name="data[bq2]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"3. Casi nunca me tomo las cosas a pecho (no me perturbo con facilidad)"</td>
                            <td><input type="radio" name="data[bq3]" value="1" required/></td>
                            <td><input type="radio" name="data[bq3]" value="2"/></td>
                            <td><input type="radio" name="data[bq3]" value="3"/></td>
                            <td><input type="radio" name="data[bq3]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"4. Mis pensamientos pueden tener gran velocidad (tengo pensamientos que van muy rápido en mi mente)"</td>
                            <td><input type="radio" name="data[bq4]" value="1" required/></td>
                            <td><input type="radio" name="data[bq4]" value="2"/></td>
                            <td><input type="radio" name="data[bq4]" value="3"/></td>
                            <td><input type="radio" name="data[bq4]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"5. Planifico mis viajes con antelación"</td>
                            <td><input type="radio" name="data[bq5]" value="1" required/></td>
                            <td><input type="radio" name="data[bq5]" value="2"/></td>
                            <td><input type="radio" name="data[bq5]" value="3"/></td>
                            <td><input type="radio" name="data[bq5]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"6. Soy una persona con autocontrol"</td>
                            <td><input type="radio" name="data[bq6]" value="1" required/></td>
                            <td><input type="radio" name="data[bq6]" value="2"/></td>
                            <td><input type="radio" name="data[bq6]" value="3"/></td>
                            <td><input type="radio" name="data[bq6]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"7. Me concentro con facilidad (se me hace fácil concentrarme)"</td>
                            <td><input type="radio" name="data[bq7]" value="1" required/></td>
                            <td><input type="radio" name="data[bq7]" value="2"/></td>
                            <td><input type="radio" name="data[bq7]" value="3"/></td>
                            <td><input type="radio" name="data[bq7]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"8. Ahorro con regularidad"</td>
                            <td><input type="radio" name="data[bq8]" value="1" required/></td>
                            <td><input type="radio" name="data[bq8]" value="2"/></td>
                            <td><input type="radio" name="data[bq8]" value="3"/></td>
                            <td><input type="radio" name="data[bq8]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"9. Se me hace difícil estar quieto/a durante largos períodos de tiempo"</td>
                            <td><input type="radio" name="data[bq9]" value="1" required/></td>
                            <td><input type="radio" name="data[bq9]" value="2"/></td>
                            <td><input type="radio" name="data[bq9]" value="3"/></td>
                            <td><input type="radio" name="data[bq9]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"10. Pienso las cosas cuidadosamente"</td>
                            <td><input type="radio" name="data[bq10]" value="1" required/></td>
                            <td><input type="radio" name="data[bq10]" value="2"/></td>
                            <td><input type="radio" name="data[bq10]" value="3"/></td>
                            <td><input type="radio" name="data[bq10]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"11. Planifico para tener un trabajo fijo (me esfuerzo por asegurar que tendré dinero para pagar mis gastos)"</td>
                            <td><input type="radio" name="data[bq11]" value="1" required/></td>
                            <td><input type="radio" name="data[bq11]" value="2"/></td>
                            <td><input type="radio" name="data[bq11]" value="3"/></td>
                            <td><input type="radio" name="data[bq11]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"12. Digo las cosas sin pensarlas"</td>
                            <td><input type="radio" name="data[bq12]" value="1" required/></td>
                            <td><input type="radio" name="data[bq12]" value="2"/></td>
                            <td><input type="radio" name="data[bq12]" value="3"/></td>
                            <td><input type="radio" name="data[bq12]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"13. Me gusta pensar sobre problemas complicados (me gusta pensar sobre problemas complejos)"</td>
                            <td><input type="radio" name="data[bq13]" value="1" required/></td>
                            <td><input type="radio" name="data[bq13]" value="2"/></td>
                            <td><input type="radio" name="data[bq13]" value="3"/></td>
                            <td><input type="radio" name="data[bq13]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"14. Cambio de trabajo frecuentemente (no me quedo en el mismo trabajo durante largos períodos de tiempo)"</td>
                            <td><input type="radio" name="data[bq14]" value="1" required/></td>
                            <td><input type="radio" name="data[bq14]" value="2"/></td>
                            <td><input type="radio" name="data[bq14]" value="3"/></td>
                            <td><input type="radio" name="data[bq14]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"15. Actúo impulsivamente"</td>
                            <td><input type="radio" name="data[bq15]" value="1" required/></td>
                            <td><input type="radio" name="data[bq15]" value="2"/></td>
                            <td><input type="radio" name="data[bq15]" value="3"/></td>
                            <td><input type="radio" name="data[bq15]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"16. Me aburro con facilidad tratando de resolver problemas en mi mente (me aburre pensar en algo por demasiado tiempo)"</td>
                            <td><input type="radio" name="data[bq16]" value="1" required/></td>
                            <td><input type="radio" name="data[bq16]" value="2"/></td>
                            <td><input type="radio" name="data[bq16]" value="3"/></td>
                            <td><input type="radio" name="data[bq16]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"17. Visito al médico y al dentista con regularidad"</td>
                            <td><input type="radio" name="data[bq17]" value="1" required/></td>
                            <td><input type="radio" name="data[bq17]" value="2"/></td>
                            <td><input type="radio" name="data[bq17]" value="3"/></td>
                            <td><input type="radio" name="data[bq17]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"18. Hago las cosas en el momento que se me ocurren"</td>
                            <td><input type="radio" name="data[bq18]" value="1" required/></td>
                            <td><input type="radio" name="data[bq18]" value="2"/></td>
                            <td><input type="radio" name="data[bq18]" value="3"/></td>
                            <td><input type="radio" name="data[bq18]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"19. Soy una persona que piensa sin distraerse (puedo enfocar mi mente en una sola cosa por mucho tiempo)"</td>
                            <td><input type="radio" name="data[bq19]" value="1" required/></td>
                            <td><input type="radio" name="data[bq19]" value="2"/></td>
                            <td><input type="radio" name="data[bq19]" value="3"/></td>
                            <td><input type="radio" name="data[bq19]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"20. Cambio de vivienda a menudo (me mudo con frecuencia o no me gusta vivir en el mismo sitio por mucho tiempo)"</td>
                            <td><input type="radio" name="data[bq20]" value="1" required/></td>
                            <td><input type="radio" name="data[bq20]" value="2"/></td>
                            <td><input type="radio" name="data[bq20]" value="3"/></td>
                            <td><input type="radio" name="data[bq20]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"21. Compro cosas impulsivamente"</td>
                            <td><input type="radio" name="data[bq21]" value="1" required/></td>
                            <td><input type="radio" name="data[bq21]" value="2"/></td>
                            <td><input type="radio" name="data[bq21]" value="3"/></td>
                            <td><input type="radio" name="data[bq21]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"22. Termino lo que empiezo"</td>
                            <td><input type="radio" name="data[bq22]" value="1" required/></td>
                            <td><input type="radio" name="data[bq22]" value="2"/></td>
                            <td><input type="radio" name="data[bq22]" value="3"/></td>
                            <td><input type="radio" name="data[bq22]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"23. Camino y me muevo con rapidez"</td>
                            <td><input type="radio" name="data[bq23]" value="1" required/></td>
                            <td><input type="radio" name="data[bq23]" value="2"/></td>
                            <td><input type="radio" name="data[bq23]" value="3"/></td>
                            <td><input type="radio" name="data[bq23]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"24. Resuelvo los problemas experimentando (resuelvo los problemas empleando una posible solución y viendo si funciona)"</td>
                            <td><input type="radio" name="data[bq24]" value="1" required/></td>
                            <td><input type="radio" name="data[bq24]" value="2"/></td>
                            <td><input type="radio" name="data[bq24]" value="3"/></td>
                            <td><input type="radio" name="data[bq24]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"25. Gasto en efectivo o a crédito más de lo que gano (gasto más de lo gano)"</td>
                            <td><input type="radio" name="data[bq25]" value="1" required/></td>
                            <td><input type="radio" name="data[bq25]" value="2"/></td>
                            <td><input type="radio" name="data[bq25]" value="3"/></td>
                            <td><input type="radio" name="data[bq25]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"26. Hablo rápido"</td>
                            <td><input type="radio" name="data[bq26]" value="1" required/></td>
                            <td><input type="radio" name="data[bq26]" value="2"/></td>
                            <td><input type="radio" name="data[bq26]" value="3"/></td>
                            <td><input type="radio" name="data[bq26]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"27. Tengo pensamientos extraños cuando estoy pensando (a veces tengo pensamientos irrelevantes cuando pienso)"</td>
                            <td><input type="radio" name="data[bq27]" value="1" required/></td>
                            <td><input type="radio" name="data[bq27]" value="2"/></td>
                            <td><input type="radio" name="data[bq27]" value="3"/></td>
                            <td><input type="radio" name="data[bq27]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"28. Me interesa más el presente que el futuro"</td>
                            <td><input type="radio" name="data[bq28]" value="1" required/></td>
                            <td><input type="radio" name="data[bq28]" value="2"/></td>
                            <td><input type="radio" name="data[bq28]" value="3"/></td>
                            <td><input type="radio" name="data[bq28]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"29. Me siento inquieto/a en clases o charlas (me siento inquieto/a si tengo que oír a alguien hablar durante un largo período de tiempo)"</td>
                            <td><input type="radio" name="data[bq29]" value="1" required/></td>
                            <td><input type="radio" name="data[bq29]" value="2"/></td>
                            <td><input type="radio" name="data[bq29]" value="3"/></td>
                            <td><input type="radio" name="data[bq29]" value="4"/></td>
                        </tr>
                        <tr>
                            <td>"30. Planifico el futuro (me interesa más el futuro que el presente)"</td>
                            <td><input type="radio" name="data[bq30]" value="1" required/></td>
                            <td><input type="radio" name="data[bq30]" value="2"/></td>
                            <td><input type="radio" name="data[bq30]" value="3"/></td>
                            <td><input type="radio" name="data[bq30]" value="4"/></td>
                        </tr>
                    </tbody>
            </table>
            <input type="submit" value="Siguiente"/>
        </div>

        </ActionForm>
    }
}
