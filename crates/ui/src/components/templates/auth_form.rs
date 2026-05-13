use crate::components::common::{ActiveScreen, classes};
use sycamore::prelude::*;

use crate::components::{
    common::ScreenWrapper,
    features::form_tabs::*,
    pages::{login_panel::*, register_panel::*},
};

#[derive(Props)]
pub struct AuthFormProps {
    visible: MaybeDyn<bool>,
}

#[component]
pub fn AuthForm(props: AuthFormProps) -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();

    view! {
        div(
            class=classes(vec![
                "container".into(),
                ("active", props.visible.clone()).into(),
            ]),
        ) {
            FormTabs()
            LoginPanel(is_active=(move || screen_wrapper.is_login()).into())
            RegisterPanel(is_active=(move || screen_wrapper.is_register()).into())
        }
    }
}
