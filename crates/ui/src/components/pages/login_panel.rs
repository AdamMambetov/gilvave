use sycamore::{prelude::*, web::events::SubmitEvent};

use crate::components::common::class_name;
use crate::components::features::form_options::FormOptions;
use crate::components::features::social_buttons::SocialButtons;
use crate::components::ui::divider::Divider;
use crate::components::ui::input_group::InputGroup;
use crate::components::ui::submit_button::SubmitButton;

#[derive(Props)]
pub struct LoginFormProps {
    is_active: Box<dyn Fn() -> bool>,
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
                    label="Email или телефон".to_string(),
                    r#type="text".to_string(),
                    placeholder="adam@mail.com".to_string(),
                    bind_value=email,
                    // on_input=&update_email,
                    error_condition=Box::new(move || email_error.get()),
                    error_message="Введите email или номер телефона".to_string(),
                )

                InputGroup(
                    label="Пароль".to_string(),
                    r#type="password".to_string(),
                    placeholder="••••••••".to_string(),
                    bind_value=password,
                    // on_input=&update_password,
                    error_condition=Box::new(move || password_error.get()),
                    error_message="Введите пароль".to_string(),
                )

                FormOptions(bind_checked=remember)

                SubmitButton(label="Войти".to_string())

                Divider(text="или войдите через".to_string())

                SocialButtons()
            }
        }
    }
}
