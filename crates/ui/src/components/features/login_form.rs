use std::rc::Rc;

use sycamore::{prelude::*, web::events::SubmitEvent};

use crate::app::LoginMode;

#[derive(Props)]
pub struct LoginFormProps {
    is_active: Rc<Signal<bool>>,
}

#[component]
pub fn LoginForm() -> View {
    let login_mode = use_context::<LoginMode>();
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
        div(class=if login_mode.is_login() { "form-panel active" } else { "form-panel" }) {
            h2 { "Добро пожаловать!" }
            form(on:submit=on_submit) {
                div(class=if email_error.get() { "input-group error" } else { "input-group" }) {
                    label { "Email или телефон" }
                    input(
                        r#type="text",
                        placeholder="example@mail.com",
                        bind:value=email,
                        // on:input=update_email,
                    )
                    div(class="error-message") { "Введите email или номер телефона" }
                }

                div(class=if password_error.get() { "input-group error" } else { "input-group" }) {
                    label { "Пароль" }
                    input(
                        r#type="password",
                        placeholder="••••••••",
                        bind:value=password,
                        // on:input=update_password,
                    )
                    div(class="error-message") { "Введите пароль" }
                }

                div(class="form-options") {
                    label(class="checkbox") {
                        input(r#type="checkbox", bind:checked=remember,
                        // on:input=toggle_remember,
                    )
                        " Запомнить меня"
                    }
                    a(href="#", class="forgot-link") { "Забыли пароль?" }
                }

                button(r#type="submit", class="submit-btn") { "Войти" }

                div(class="divider") {
                    span { "или войдите через" }
                }

                div(class="social-buttons") {
                    button(
                        r#type="button",
                        class="social-btn",
                        on:click=|_| {
                            window()
                                .alert_with_message("🔐 Вход через Google (демо-режим)")
                                .unwrap();
                        },
                    ) { "Google" }
                    button(
                        r#type="button",
                        class="social-btn",
                        on:click=|_| {
                            window()
                                .alert_with_message("🔐 Вход через GitHub (демо-режим)")
                                .unwrap();
                        },
                    ) { "GitHub" }
                }
            }
        }
    }
}
