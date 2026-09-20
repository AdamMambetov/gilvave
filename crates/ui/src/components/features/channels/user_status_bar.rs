use sycamore::prelude::*;

use crate::components::common::{UiModalContext, UserProfileContext};

#[component]
pub fn UserStatusBar() -> View {
    let user_profile = use_context::<UserProfileContext>();
    let modal_context = use_context::<UiModalContext>();

    let on_toggle_mic = move |_| {
        user_profile.is_muted.set(!user_profile.is_muted.get());
    };

    let on_toggle_deafen = move |_| {
        user_profile.is_deafened.set(!user_profile.is_deafened.get());
    };

    let on_open_settings = move |_| {
        modal_context.is_profile_settings_open.set(true);
    };

    let avatar_view = move || {
        let av = user_profile.avatar.get_clone();
        let name = user_profile.username.get_clone();
        let initial = name.chars().next().unwrap_or('?').to_uppercase().to_string();

        if !av.is_empty() {
            view! {
                div(class="footer-avatar") {
                    img(src=av, alt="")
                    div(class="status-dot online")
                }
            }
        } else {
            view! {
                div(class="footer-avatar placeholder") {
                    span { (initial) }
                    div(class="status-dot online")
                }
            }
        }
    };

    view! {
        div(class="channel-panel-footer") {
            div(class="footer-user", on:click=on_open_settings, title="Настройки профиля") {
                (avatar_view())
                div(class="footer-user-info") {
                    div(class="footer-username") { (user_profile.username.get_clone()) }
                    div(class="footer-discriminator") { "@" (user_profile.username.get_clone().to_lowercase()) }
                }
            }

            div(class="footer-controls") {
                button(
                    class=if user_profile.is_muted.get() { "footer-btn muted" } else { "footer-btn" },
                    on:click=on_toggle_mic,
                    title=if user_profile.is_muted.get() { "Включить микрофон" } else { "Отключить микрофон" },
                ) {
                    (if user_profile.is_muted.get() {
                        "🔇"
                    } else {
                        "🎙️"
                    })
                }

                button(
                    class=if user_profile.is_deafened.get() { "footer-btn deafened" } else { "footer-btn" },
                    on:click=on_toggle_deafen,
                    title=if user_profile.is_deafened.get() { "Включить звук" } else { "Заглушить звук" },
                ) {
                    "🎧"
                }

                button(
                    class="footer-btn settings",
                    on:click=on_open_settings,
                    title="Настройки пользователя",
                ) {
                    "⚙️"
                }
            }
        }
    }
}
