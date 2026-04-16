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
                label="Вход".to_string(),
                on_click=Box::new(move || if !login_mode.is_login() { switch_form() } ),
                is_active=Box::new(move || login_mode.is_login()))
            TabButton(
                label="Регистрация".to_string(),
                on_click=Box::new(move || if login_mode.is_login() { switch_form() } ),
                is_active=Box::new(move || !login_mode.is_login()))
        }
    }
}
