use std::rc::Rc;
use sycamore::{prelude::*, web::events::SubmitEvent};

use crate::app::LoginMode;

#[derive(Props)]
pub struct RegisterFormProps {
    is_inactive: Rc<Signal<bool>>,
}

#[component]
pub fn RegisterForm() -> View {
    let login_mode = use_context::<LoginMode>();
    let name = create_signal(String::new());
    let email = create_signal(String::new());
    let password = create_signal(String::new());
    let confirm_password = create_signal(String::new());
    let terms = create_signal(false);

    let name_error = create_signal(false);
    let email_error = create_signal(false);
    let password_error = create_signal(false);
    let confirm_error = create_signal(false);

    let validate_email = |email: &str| -> bool {
        let re = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
        re.is_match(email)
    };

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let mut is_valid = true;

        if name.get_clone().trim().len() < 2 {
            name_error.set(true);
            is_valid = false;
        } else {
            name_error.set(false);
        }

        if !validate_email(&email.get_clone()) {
            email_error.set(true);
            is_valid = false;
        } else {
            email_error.set(false);
        }

        if password.get_clone().len() < 6 {
            password_error.set(true);
            is_valid = false;
        } else {
            password_error.set(false);
        }

        if password.get_clone() != confirm_password.get_clone() {
            confirm_error.set(true);
            is_valid = false;
        } else {
            confirm_error.set(false);
        }

        if !terms.get() {
            window()
                .alert_with_message("⚠️ Пожалуйста, примите условия использования")
                .unwrap();
            is_valid = false;
        }

        if is_valid {
            console_log!(
                "Регистрация: name={}, email={}",
                name.get_clone(),
                email.get_clone()
            );
            window()
                .alert_with_message("✅ Регистрация прошла успешно! (демо-режим)")
                .unwrap();
        }
    };

    // let update_name = move |event: web_sys::Event| {
    //     let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
    //     name.set(input.value());
    //     name_error.set(false);
    // };

    // let update_email = move |event: web_sys::Event| {
    //     let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
    //     email.set(input.value());
    //     email_error.set(false);
    // };

    // let update_password = move |event: web_sys::Event| {
    //     let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
    //     password.set(input.value());
    //     password_error.set(false);
    //     confirm_error.set(false);
    // };

    // let update_confirm = move |event: web_sys::Event| {
    //     let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
    //     confirm_password.set(input.value());
    //     confirm_error.set(false);
    // };

    // let toggle_terms = move |event: web_sys::Event| {
    //     let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
    //     terms.set(input.checked());
    // };

    view! {
        div(class=if !login_mode.is_login() { "form-panel active" } else { "form-panel" }) {
            h2 { "Создать аккаунт" }
            form(on:submit=on_submit) {
                div(class=if name_error.get() { "input-group error" } else { "input-group" }) {
                    label { "Имя пользователя" }
                    input(
                        r#type="text",
                        placeholder="Иван Иванов",
                        bind:value=name,
                        // on:input=update_name,
                    )
                    div(class="error-message") { "Введите имя (минимум 2 символа)" }
                }

                div(class=if email_error.get() { "input-group error" } else { "input-group" }) {
                    label { "Email" }
                    input(
                        r#type="email",
                        placeholder="example@mail.com",
                        bind:value=email,
                        // on:input=update_email,
                    )
                    div(class="error-message") { "Введите корректный email" }
                }

                div(class=if password_error.get() { "input-group error" } else { "input-group" }) {
                    label { "Пароль" }
                    input(
                        r#type="password",
                        placeholder="••••••••",
                        bind:value=password,
                        // on:input=update_password,
                    )
                    div(class="error-message") { "Пароль должен быть не менее 6 символов" }
                }

                div(class=if confirm_error.get() { "input-group error" } else { "input-group" }) {
                    label { "Подтверждение пароля" }
                    input(
                        r#type="password",
                        placeholder="••••••••",
                        bind:value=confirm_password,
                        // on:input=update_confirm,
                    )
                    div(class="error-message") { "Пароли не совпадают" }
                }

                div(class="form-options") {
                    label(class="checkbox") {
                        input(
                            r#type="checkbox",
                            bind:checked=terms,
                            // on:input=toggle_terms,
                        )
                        " Я согласен с "
                        a(href="#", class="forgot-link") { "условиями использования" }
                    }
                }

                button(r#type="submit", class="submit-btn") { "Зарегистрироваться" }

                div(class="divider") {
                    span { "или зарегистрируйтесь через" }
                }

                div(class="social-buttons") {
                    button(
                        r#type="button",
                        class="social-btn",
                        on:click=|_| {
                            window()
                                .alert_with_message("🔐 Регистрация через Google (демо-режим)")
                                .unwrap();
                        },
                    ) { "Google" }
                    button(
                        r#type="button",
                        class="social-btn",
                        on:click=|_| {
                            window()
                                .alert_with_message("🔐 Регистрация через GitHub (демо-режим)")
                                .unwrap();
                        },
                    ) { "GitHub" }
                }
            }
        }
    }
}
