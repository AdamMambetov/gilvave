use sycamore::prelude::*;

use crate::components::common::LoginMode;
use crate::components::ui::tab_button::TabButton;

#[component]
pub fn FormTabs() -> View {
    let login_mode = use_context::<LoginMode>();
    let switch_form = move || {
        use_context::<LoginMode>().toggle();
    };

    view! {
        div(class="form-header") {
            TabButton(
                on:click=move |_| if !login_mode.is_login() { switch_form() },
                is_active=(move || login_mode.is_login()).into(),
            ) { "Вход" }

            TabButton(
                on:click=move |_| if login_mode.is_login() { switch_form() },
                is_active=(move || !login_mode.is_login()).into(),
            ) { "Регистрация" }
        }
    }
}
