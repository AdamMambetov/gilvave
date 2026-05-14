use gilvave_core::dto::user::ProfileResponse;
use sycamore::{futures::spawn_local_scoped, prelude::*};
use wasm_bindgen::JsValue;

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

    spawn_local_scoped(async move {
        let res = invoke("get_profile", JsValue::NULL).await;
        let profile = serde_wasm_bindgen::from_value::<ProfileResponse>(res);
        if profile.is_ok() {
            screen_wrapper.set(ActiveScreen::Home);
        }
    });

    view! {
        main() {
            AuthForm()
            HomePanel()
        }
    }
}
