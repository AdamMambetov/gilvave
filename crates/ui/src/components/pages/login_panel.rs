use sycamore::{prelude::*, web::events::SubmitEvent};

use crate::components::common::class_name;
use crate::components::features::social_buttons::SocialButtons;
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::divider::Divider;
use crate::components::ui::input_group::InputGroup;
use crate::components::ui::submit_button::SubmitButton;

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
            window()
                .alert_with_message("✅ Выполняется вход... (демо-режим)")
                .unwrap();
        } else {
            console_log!(
                "error: email={}, password={}",
                email_error.get(),
                password_error.get()
            );
        }
    };

    // let update_email = move |event: web_sys::Event| {
    //     let input: HtmlInput = event.target().unwrap().into();
    //     email.set(input.value());
    //     email_error.set(false);
    // };

    // let update_password = move |event: web_sys::Event| {
    //     let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
    //     password.set(input.value());
    //     password_error.set(false);
    // };

    // let toggle_remember = move |event: web_sys::Event| {
    //     let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
    //     remember.set(input.checked());
    // };

    view! {
        div(class=class_name("form-panel", &props.is_active, "active", "")) {
            h2 { "Добро пожаловать!" }
            form(on:submit=on_submit) {
                InputGroup(
                    r#type="text",
                    placeholder="adam@mail.com",
                    bind:value=email,
                    // on:input=&update_email,
                    label="Email или телефон",
                    is_error=email_error.into(),
                    error_message="Введите email или номер телефона",
                )

                InputGroup(
                    r#type="password",
                    placeholder="••••••••",
                    bind:value=password,
                    // on:input=&update_password,
                    label="Пароль",
                    is_error=password_error.into(),
                    // is_error=Box::new(move || password_error.get()),
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
