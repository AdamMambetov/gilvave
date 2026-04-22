use gilvave_core::dto::user::{AuthTokensResponse, LoginRequest, ProfileResponse};
use sycamore::futures::spawn_local_scoped;
use sycamore::web::rt::web_sys::console;
use sycamore::{prelude::*, web::events::SubmitEvent};
use wasm_bindgen::JsValue;

use crate::components::common::classes;
use crate::components::features::social_buttons::SocialButtons;
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::divider::Divider;
use crate::components::ui::input_group::InputGroup;
use crate::components::ui::submit_button::SubmitButton;
use crate::utils::invoke;

#[derive(Props)]
pub struct LoginFormProps {
    is_active: MaybeDyn<bool>,
}

#[component]
pub fn LoginPanel(props: LoginFormProps) -> View {
    let email = create_signal(String::new());
    let password = create_signal(String::new());
    let remember = create_signal(false);
    let email_error = create_signal(false);
    let password_error = create_signal(false);

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let mut is_valid = true;

        if email.get_clone().trim().is_empty() {
            email_error.set(true);
            is_valid = false;
        } else {
            email_error.set(false);
        }

        if password.get_clone().is_empty() {
            password_error.set(true);
            is_valid = false;
        } else {
            password_error.set(false);
        }

        if is_valid {
            console_log!(
                "Вход: email={}, password={}",
                email.get_clone(),
                password.get_clone()
            );
            // window()
            //     .alert_with_message("✅ Выполняется вход... (демо-режим)")
            //     .unwrap();

            spawn_local_scoped(async move {
                let args = serde_wasm_bindgen::to_value(&LoginRequest {
                    email: email.to_string(),
                    password: password.to_string(),
                })
                .unwrap();
                let value = invoke("login", args).await;
                let login_response =
                    serde_wasm_bindgen::from_value::<AuthTokensResponse>(value).unwrap();

                console_log!(
                    r#"login response:
                    access_token - {}
                    refresh_token - {}"#,
                    login_response.access_token,
                    login_response.refresh_token,
                );

                let value = invoke("get_profile", JsValue::NULL).await;
                let profile_response =
                    serde_wasm_bindgen::from_value::<ProfileResponse>(value).unwrap();

                console_log!("{:?}", profile_response);
            });
        } else {
            console_log!(
                "error: email={}, password={}",
                email_error.get(),
                password_error.get()
            );
        }
    };

    view! {
        div(
            class=classes(vec![
                "form-panel".into(),
                ("active", props.is_active.clone()).into(),
            ]),
        ) {
            h2 { "Добро пожаловать!" }
            form(on:submit=on_submit) {
                InputGroup(
                    r#type="text",
                    placeholder="adam@mail.com",
                    bind:value=email,
                    label="Email или телефон",
                    is_error=email_error.into(),
                    error_message="Введите email или номер телефона",
                )

                InputGroup(
                    r#type="password",
                    placeholder="••••••••",
                    bind:value=password,
                    label="Пароль",
                    is_error=password_error.into(),
                    error_message="Введите пароль",
                )

                div(class="form-options") {
                    Checkbox(bind:checked=remember) {
                        "Запомнить меня"
                    }
                    a(href="#", class="forgot-link") { "Забыли пароль?" }
                }

                SubmitButton() { ("Войти") }

                Divider() { ("или войдите через") }

                SocialButtons()
            }
        }
    }
}
