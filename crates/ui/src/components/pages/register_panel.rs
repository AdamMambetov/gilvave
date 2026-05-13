use std::collections::HashMap;

use gilvave_core::dto::user::{RegisterRequest, RegisterResponse};
use serde::Deserialize;
use sycamore::{futures::spawn_local_scoped, prelude::*, web::events::SubmitEvent};
use validator::Validate;

use crate::{
    components::{
        common::classes,
        features::social_buttons::SocialButtons,
        ui::{divider::Divider, input_group::InputGroup, submit_button::SubmitButton},
    },
    utils::invoke,
};

#[derive(Props)]
pub struct RegisterFormProps {
    is_active: MaybeDyn<bool>,
}

#[derive(Debug, Validate, Deserialize)]
struct RegisterData {
    #[validate(length(min = 2, max = 50))]
    name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6, max = 50))]
    password: String,
    #[validate(must_match(other = "password"))]
    confirm_password: String,
}

#[component]
pub fn RegisterPanel(props: RegisterFormProps) -> View {
    let name = create_signal(String::new());
    let email = create_signal(String::new());
    let password = create_signal(String::new());
    let confirm_password = create_signal(String::new());

    let name_error = create_signal(false);
    let email_error = create_signal(false);
    let password_error = create_signal(false);
    let confirm_error = create_signal(false);

    let mut form_map: HashMap<&str, Signal<bool>> = [
        ("name", name_error),
        ("email", email_error),
        ("password", password_error),
        ("confirm_password", confirm_error),
    ]
    .into();

    let on_submit = move |event: SubmitEvent| {
        // not reset page
        event.prevent_default();

        let data = RegisterData {
            name: name.get_clone(),
            email: email.get_clone(),
            password: password.get_clone(),
            confirm_password: confirm_password.get_clone(),
        };

        match data.validate() {
            Ok(_) => {
                form_map.values_mut().for_each(|signal| signal.set(false));

                spawn_local_scoped(async move {
                    // let args = serde_wasm_bindgen::to_value("ws://26.186.139.15:3000/ws").unwrap();
                    // invoke("start_websocket_listener", args).await;

                    let args = serde_wasm_bindgen::to_value(&RegisterRequest {
                        username: data.name.to_string(),
                        email: data.email.to_string(),
                        password: data.password.to_string(),
                    })
                    .unwrap();
                    let value = invoke("register", args).await;
                    let register_response =
                        serde_wasm_bindgen::from_value::<RegisterResponse>(value).unwrap();

                    console_log!(
                        r#"register response:
                        id - {}
                        username - {}
                        email - {}"#,
                        register_response.id,
                        register_response.username,
                        register_response.email,
                    );
                });
            }
            Err(errors) => {
                form_map.values_mut().for_each(|signal| signal.set(false));
                for (field, _) in errors.field_errors() {
                    form_map[field.into_owned().as_str()].set(true);
                }
            }
        }
    };

    view! {
        div(
            class=classes(vec![
                "form-panel".into(),
                ("active", props.is_active.clone()).into(),
            ]),
        ) {
            h2 { "Создать аккаунт" }
            form(on:submit=on_submit) {
                InputGroup(
                    r#type="text",
                    placeholder="Ислам",
                    bind:value=name,
                    label="Имя пользователя",
                    is_error=name_error.into(),
                    error_message="Введите имя (минимум 2 символа)",
                )

                InputGroup(
                    r#type="email",
                    placeholder="example@mail.com",
                    bind:value=email,
                    label="Email",
                    is_error=email_error.into(),
                    error_message="Введите корректный email",
                )

                InputGroup(
                    r#type="password",
                    placeholder="••••••••",
                    bind:value=password,
                    label="Пароль",
                    is_error=password_error.into(),
                    error_message="Пароль должен быть не менее 6 символов",
                )

                InputGroup(
                    r#type="password",
                    placeholder="••••••••",
                    bind:value=confirm_password,
                    label="Подтверждение пароля",
                    is_error=confirm_error.into(),
                    error_message="Пароли не совпадают",
                )

                a(href="#", class="forgot-link") {
                    "Нажимая кнопку «Зарегистрироваться», вы соглашаетесь с Условиями использования Gilvave"
                }

                SubmitButton(on:click=move |_| console_log!("{name}")) { "Зарегистрироваться" }

                Divider() { "или зарегистрируйтесь через" }

                SocialButtons()
            }
        }
    }
}
