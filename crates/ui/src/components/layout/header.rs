use strum::IntoEnumIterator;
use sycamore::{futures::spawn_local_scoped, prelude::*, web::events::Event};
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

use crate::components::common::{ActiveScreen, ScreenWrapper};

#[component]
pub fn AppHeader() -> View {
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

    let app_name = create_signal(String::default());
    spawn_local_scoped(async move {
        app_name.set(tauri_sys::app::get_name().await);
    });

    view! {
        div(class="app-header", on:mousedown=on_drag) {
            div(class="app-header-left") {
                ScreenSelect()
                span(class="app-header-title") { (app_name) }
            }

            div(class="window-controls") {
                button(class="window-btn", on:click=move |_| invoke_window("window_minimize")) { "─" }
                button(class="window-btn", on:click=move |_| invoke_window("window_toggle_maximize")) { "□" }
                button(class="window-btn close", on:click=move |_| invoke_window("window_close")) { "✕" }
            }
        }
    }
}

#[cfg(not(debug_assertions))]
#[component]
fn ScreenSelect() -> View {
    view! {}
}

#[cfg(debug_assertions)]
#[component]
fn ScreenSelect() -> View {
    let screens = ActiveScreen::iter().collect::<Vec<_>>();
    let screen_wrapper = use_context::<ScreenWrapper>();
    let select_ref = create_node_ref();

    let handle_change = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(select) = target.dyn_into::<HtmlSelectElement>()
            && let Ok(value) = select.value().parse::<ActiveScreen>()
        {
            screen_wrapper.set(value);
        }
    };

    create_effect(move || {
        if let Some(select_node) = select_ref.try_get() {
            let select = select_node.dyn_ref::<HtmlSelectElement>().unwrap();
            if select.value() != screen_wrapper.get().to_string() {
                select.set_value(&screen_wrapper.get().to_string());
            }
        }
    });

    view! {
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
    }
}

fn invoke_window(command: &'static str) {
    spawn_local_scoped(async move {
        tauri_sys::core::invoke::<serde_json::Value>(command, &serde_json::json!({})).await;
    });
}
