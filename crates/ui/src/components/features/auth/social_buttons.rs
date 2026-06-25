use sycamore::prelude::*;

use crate::components::ui::{icons::*, social_button::*};

#[component]
pub fn SocialButtons() -> View {
    view! {
        div(class="social-buttons") {
            SocialButton(
                on:click=move |_|
                    window()
                        .alert_with_message("🔐 Регистрация через Google (демо-режим)")
                        .unwrap(),
            ) {
                GoogleIcon()
                "Google"
            }

            SocialButton(
                on:click=move |_|
                    window()
                        .alert_with_message("🔐 Регистрация через GitHub (демо-режим)")
                        .unwrap(),
            ) {
                GitHubIcon()
                "GitHub"
            }
        }
    }
}
