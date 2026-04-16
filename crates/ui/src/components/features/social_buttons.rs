use sycamore::prelude::*;

use crate::components::ui::{icons::*, social_button::*};

#[component]
pub fn SocialButtons() -> View {
    view! {
        div(class="social-buttons") {
            SocialButton(
                label="Google".to_string(),
                icon=GoogleIcon(),
                on_click=Box::new(move ||
                    window()
                        .alert_with_message("🔐 Регистрация через Google (демо-режим)")
                        .unwrap() ),
            )
            SocialButton(
                label="GitHub".to_string(),
                icon=GitHubIcon(),
                on_click=Box::new(move ||
                    window()
                        .alert_with_message("🔐 Регистрация через GitHub (демо-режим)")
                        .unwrap() ),
            )
        }
    }
}
