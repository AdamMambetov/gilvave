use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::{
        common::{ActiveScreen, ScreenWrapper},
        layout::header::AppHeader,
        pages::home_panel::HomePanel,
        templates::auth_form::AuthForm,
    },
    http::api::Api,
};

#[component]
pub fn App() -> View {
    let screen_wrapper = ScreenWrapper(create_signal(ActiveScreen::Login));
    provide_context(screen_wrapper);

    spawn_local_scoped(async move {
        if Api::get_profile().await.is_ok() {
            screen_wrapper.set(ActiveScreen::Home);
        }
    });

    view! {
        main() {
            AppHeader()
            AuthForm()
            HomePanel()
        }
    }
}
