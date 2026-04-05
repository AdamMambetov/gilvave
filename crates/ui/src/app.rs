use gilvave_core::dto::user::{RegisterRequest, RegisterResponse};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> View {
    let name = create_signal(String::new());
    let password = create_signal(String::new());
    let request_msg = create_signal(String::new());

    let request = move |e: SubmitEvent| {
        // не сбрасываем перезагрузку страницы
        e.prevent_default();
        spawn_local_scoped(async move {
            let args = serde_wasm_bindgen::to_value(&RegisterRequest {
                username: name.get_clone(),
                password: password.get_clone(),
            })
            .unwrap();

            let msg = invoke("register", args).await;
            console_log!("{:?}", msg);
            let res = serde_wasm_bindgen::from_value::<RegisterResponse>(msg).unwrap();
            request_msg.set(format_args!("id: {}, username: {}", res.id, res.username).to_string());
            // let args = serde_wasm_bindgen::to_value("ws://26.186.139.15:3000/ws").unwrap();
            // invoke("start_websocket_listener", args).await;
        });
    };
    let clicked = move |_| {
        console_log!("clicked {}, {}", name.to_string(), password.to_string());
    };

    view! {
        main(class="container") {
            form(class="row", on:submit=request) {
                input(id="request-input", bind:value=name, placeholder="Enter a name...")
                input(id="request-input-pass", bind:value=password, placeholder="Enter a password...")
                button(r#type="submit") {
                    "Request"
                }
            }
            p {
                (request_msg)
            }
            button(class="btn-1", on:click=clicked) {
                "text"
            }
        }
    }
}
