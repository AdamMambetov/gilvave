use strum::IntoEnumIterator;
use sycamore::{futures::spawn_local_scoped, prelude::*, web::events::Event};
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

use crate::components::common::{ActiveScreen, ScreenWrapper};

fn invoke_window(command: &'static str) {
    spawn_local_scoped(async move {
        let _ = tauri_sys::core::invoke::<serde_json::Value>(command, &serde_json::json!({})).await;
    });
}

#[component]
pub fn AppHeader() -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();
    let screens = ActiveScreen::iter().collect::<Vec<_>>();

    let on_drag = move |e: web_sys::MouseEvent| {
        if let Some(target) = e.target() {
            if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                if el.tag_name() == "SELECT" || el.tag_name() == "BUTTON" {
                    return;
                }
            }
        }
        invoke_window("window_start_dragging");
    };

    let handle_change = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(select) = target.dyn_into::<HtmlSelectElement>()
            && let Ok(value) = select.value().parse::<ActiveScreen>()
        {
            screen_wrapper.set(value);
        }
    };

    let select_ref = create_node_ref();

    create_effect(move || {
        if let Some(select_node) = select_ref.try_get() {
            let select = select_node.dyn_ref::<HtmlSelectElement>().unwrap();
            if select.value() != screen_wrapper.get().to_string() {
                select.set_value(&screen_wrapper.get().to_string());
            }
        }
    });

    view! {
        div(class="app-header", on:mousedown=on_drag) {
            div(class="app-header-left") {
                select(
                    r#ref=select_ref,
                    class="screen-select",
                    on:change=handle_change,
                ) {
                    Indexed(
                        list=screens,
                        view=|screen| { view! {
                            option(value=screen.to_string()) { (screen.to_string()) }
                        }}
                    )
                }
                span(class="app-header-title") { "gilvave" }
            }

            div(class="window-controls") {
                button(class="window-btn", on:click=move |_| invoke_window("window_minimize")) { "─" }
                button(class="window-btn", on:click=move |_| invoke_window("window_toggle_maximize")) { "□" }
                button(class="window-btn close", on:click=move |_| invoke_window("window_close")) { "✕" }
            }
        }
    }
}
