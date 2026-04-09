use crate::components::features::login_form::LoginForm;
use crate::components::features::register_form::RegisterForm;
use crate::components::ui::tab_button::TabButton;

use gilvave_core::dto::user::{RegisterRequest, RegisterResponse};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LoginMode(Signal<bool>);

impl LoginMode {
    pub fn is_login(self) -> bool {
        self.0.get()
    }

    pub fn toggle(self) {
        self.0.set(!self.0.get());
    }
}

#[component]
pub fn App() -> View {
    // let name = create_signal(String::new());
    // let password = create_signal(String::new());
    // let request_msg = create_signal(String::new());
    let login_mode = LoginMode(create_signal(true));
    provide_context(login_mode);

    let switch_form = move || {
        use_context::<LoginMode>().toggle();
    };

    // let request = move |e: SubmitEvent| {
    //     // не сбрасываем перезагрузку страницы
    //     e.prevent_default();
    //     spawn_local_scoped(async move {
    //         let args = serde_wasm_bindgen::to_value(&RegisterRequest {
    //             username: name.get_clone(),
    //             password: password.get_clone(),
    //         })
    //         .unwrap();

    //         let msg = invoke("register", args).await;
    //         console_log!("{:?}", msg);
    //         let res = serde_wasm_bindgen::from_value::<RegisterResponse>(msg).unwrap();
    //         request_msg.set(format_args!("id: {}, username: {}", res.id, res.username).to_string());
    //         // let args = serde_wasm_bindgen::to_value("ws://26.186.139.15:3000/ws").unwrap();
    //         // invoke("start_websocket_listener", args).await;
    //     });
    // };
    // let clicked = move |_| {
    // console_log!("clicked {}, {}", name.to_string(), password.to_string());
    // };

    view! {
        main(class="container") {
            div(class="container") {
                div(class="form-header") {
                    TabButton(
                        label="Вход".to_string(),
                        on_click=Box::new(move || if !login_mode.is_login() { switch_form() } ),
                        is_active=Box::new(move || login_mode.is_login()))
                    TabButton(
                        label="Регистрация".to_string(),
                        on_click=Box::new(move || if login_mode.is_login() { switch_form() } ),
                        is_active=Box::new(move || !login_mode.is_login()))
                }

                LoginForm()
                RegisterForm()
            }
        }
    }
}
