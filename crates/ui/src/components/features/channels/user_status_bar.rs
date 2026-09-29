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
        let av_opt = user_profile
            .avatar
            .with(|av| (!av.is_empty()).then(|| av.clone()));
        if let Some(av) = av_opt {
            view! {
                div(class="footer-avatar") {
                    img(src=av, alt="")
                    div(class="status-dot online")
                }
            }
        } else {
            let initial = user_profile.username.with(|name| {
                name.chars()
                    .next()
                    .unwrap_or('?')
                    .to_uppercase()
                    .to_string()
            });
            view! {
                div(class="footer-avatar placeholder") {
                    span { (initial) }
                    div(class="status-dot online")
                }
            }
        }
    };

    let username_text = move || user_profile.username.get_clone();
    let discriminator_text = move || {
        let lower = user_profile.username.with(|u| u.to_lowercase());
        format!("@{lower}")
    };

    let mic_btn_class = move || {
        if user_profile.is_muted.get() {
            "footer-btn muted"
        } else {
            "footer-btn"
        }
    };
    let mic_btn_title = move || {
        if user_profile.is_muted.get() {
            "Включить микрофон"
        } else {
            "Отключить микрофон"
        }
    };
    let mic_btn_icon = move || {
        if user_profile.is_muted.get() {
            "🔇"
        } else {
            "🎙️"
        }
    };
    let deafen_btn_class = move || {
        if user_profile.is_deafened.get() {
            "footer-btn deafened"
        } else {
            "footer-btn"
        }
    };
    let deafen_btn_title = move || {
        if user_profile.is_deafened.get() {
            "Включить звук"
        } else {
            "Заглушить звук"
        }
    };

    view! {
        div(class="channel-panel-footer") {
            div(class="footer-user", on:click=on_open_settings, title="Настройки профиля") {
                (avatar_view())
                div(class="footer-user-info") {
                    div(class="footer-username") { (username_text()) }
                    div(class="footer-discriminator") { (discriminator_text()) }
                }
            }

            div(class="footer-controls") {
                button(
                    class=mic_btn_class,
                    on:click=on_toggle_mic,
                    title=mic_btn_title,
                ) {
                    (mic_btn_icon())
                }

                button(
                    class=deafen_btn_class,
                    on:click=on_toggle_deafen,
                    title=deafen_btn_title,
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

