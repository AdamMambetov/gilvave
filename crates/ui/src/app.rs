use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

use crate::components::templates::auth_form::AuthForm;
// use gilvave_core::dto::user::{RegisterRequest, RegisterResponse};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> View {
    // let name = create_signal(String::new());
    // let password = create_signal(String::new());
    // let request_msg = create_signal(String::new());
    //
    // let request = move |e: SubmitEvent| {
    //     // не сбрасываем перезагрузку страницы
    //     e.prevent_default();
    //     spawn_local_scoped(async move {
    //         let args = serde_wasm_bindgen::to_value(&RegisterRequest {
    //             username: name.get_clone(),
    //             password: password.get_clone(),
    //         })
    //         .unwrap();
    //
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
            AuthForm()
        }
    }
}
