use gilvave_core::dto::command::CommandArgs;
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::{
        common::{ActiveScreen, ScreenWrapper},
        pages::home_panel::HomePanel,
        templates::auth_form::AuthForm,
        ui::header::AppHeader,
    },
    utils::invoke_command,
};

#[component]
pub fn App() -> View {
    let screen_wrapper = ScreenWrapper(create_signal(ActiveScreen::Login));
    provide_context(screen_wrapper);

    spawn_local_scoped(async move {
        let res = invoke_command(CommandArgs::GetProfile.to_json()).await;
        if res.is_ok() {
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
