use sycamore::prelude::*;

use crate::components::{
    common::LoginMode,
    features::form_tabs::*,
    pages::{login_panel::*, register_panel::*},
};

#[component]
pub fn AuthForm() -> View {
    let login_mode = LoginMode(create_signal(true));
    provide_context(login_mode);

    view! {
        div(class="container") {
            FormTabs()
            LoginPanel(is_active=(move || login_mode.is_login()).into())
            RegisterPanel(is_active=(move || !login_mode.is_login()).into())
        }
    }
}
