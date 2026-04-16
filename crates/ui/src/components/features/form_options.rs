use sycamore::prelude::*;

use crate::components::ui::checkbox::Checkbox;

#[derive(Props)]
pub struct FormOptionsProps {
    bind_checked: Signal<bool>,
}

#[component]
pub fn FormOptions(props: FormOptionsProps) -> View {
    view! {
        div(class="form-options") {
            Checkbox(
                label="Запомнить меня".to_string(),
                bind_checked=props.bind_checked,
            )
            a(href="#", class="forgot-link") { "Забыли пароль?" }
        }
    }
}
