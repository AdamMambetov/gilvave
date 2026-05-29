use gilvave_core::dto::user::UserView;
use strum::IntoEnumIterator;
use sycamore::{futures::spawn_local_scoped, prelude::*, web::events::Event};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlSelectElement;

use crate::{
    components::{
        common::{ActiveScreen, ScreenWrapper},
        pages::home_panel::HomePanel,
        templates::auth_form::AuthForm,
    },
    utils::invoke,
};

#[component]
pub fn App() -> View {
    let screen_wrapper = ScreenWrapper(create_signal(ActiveScreen::Login));
    provide_context(screen_wrapper);
    let screens = ActiveScreen::iter().collect::<Vec<_>>();

    spawn_local_scoped(async move {
        let res = invoke("get_profile", JsValue::NULL).await;
        let profile = serde_wasm_bindgen::from_value::<UserView>(res);
        if profile.is_ok() {
            screen_wrapper.set(ActiveScreen::Home);
        }
    });

    let handle_change = move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(select) = target.dyn_into::<HtmlSelectElement>()
            && let Ok(value) = select.value().parse::<ActiveScreen>()
        {
            screen_wrapper.set(value);
        } else {
            eprintln!("Ошибка: событие пришло не от <select> элемента");
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
        main() {
            select(r#ref=select_ref, on:change=handle_change) {
                Indexed(
                    list=screens,
                    view=|screen| { view! { option(value=screen.to_string()) { (screen.to_string()) } } }
                )
            }
            AuthForm()
            HomePanel()
        }
    }
}
