use sycamore::{prelude::*, web::events::SubmitEvent};

use crate::components::{
    common::class_name,
    features::social_buttons::SocialButtons,
    ui::{divider::Divider, input_group::InputGroup, submit_button::SubmitButton},
};

#[derive(Props)]
pub struct RegisterFormProps {
    is_active: Box<dyn Fn() -> bool>,
}

#[component]
pub fn RegisterPanel(props: RegisterFormProps) -> View {
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
        div(class=class_name("form-panel", &props.is_active, "active", "")) {
            h2 { "Создать аккаунт" }
            form(on:submit=on_submit) {
                InputGroup(
                    label="Имя пользователя".to_string(),
                    r#type="text".to_string(),
                    placeholder="Ислам Кертов".to_string(),
                    bind_value=name,
                    error_condition=Box::new(move || name_error.get()),
                    error_message="Введите имя (минимум 2 символа)".to_string(),
                )

                InputGroup(
                    label="Email".to_string(),
                    r#type="email".to_string(),
                    placeholder="example@mail.com".to_string(),
                    bind_value=email,
                    error_condition=Box::new(move || email_error.get()),
                    error_message="Введите корректный email".to_string(),
                )

                InputGroup(
                    label="Пароль".to_string(),
                    r#type="password".to_string(),
                    placeholder="••••••••".to_string(),
                    bind_value=password,
                    error_condition=Box::new(move || password_error.get()),
                    error_message="Пароль должен быть не менее 6 символов".to_string(),
                )

                InputGroup(
                    label="Подтверждение пароля".to_string(),
                    r#type="password".to_string(),
                    placeholder="••••••••".to_string(),
                    bind_value=confirm_password,
                    error_condition=Box::new(move || confirm_error.get()),
                    error_message="Пароли не совпадают".to_string(),
                )

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

                SubmitButton(label="Зарегистрироваться".to_string())

                Divider(text="или зарегистрируйтесь через".to_string())

                SocialButtons()
            }
        }
    }
}
