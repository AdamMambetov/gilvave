use sycamore::prelude::*;

use crate::components::common::{ActiveScreen, ScreenWrapper, UiModalContext, UserProfileContext, classes};

#[derive(Clone, Copy, PartialEq)]
enum SettingsTab {
    Profile,
    Security,
}

#[component]
pub fn ProfileSettingsModal() -> View {
    let modal_context = use_context::<UiModalContext>();
    let user_profile = use_context::<UserProfileContext>();
    let screen_wrapper = use_context::<ScreenWrapper>();

    let active_tab = create_signal(SettingsTab::Profile);

    let name_input = create_signal(String::new());
    let avatar_input = create_signal(String::new());
    let banner_input = create_signal(String::new());
    let bio_input = create_signal(String::new());

    let old_password = create_signal(String::new());
    let new_password = create_signal(String::new());
    let confirm_password = create_signal(String::new());
    let password_msg = create_signal(String::new());

    // Sync from user_profile when modal opens
    create_effect(move || {
        if modal_context.is_profile_settings_open.get() {
            name_input.set(user_profile.username.get_clone());
            avatar_input.set(user_profile.avatar.get_clone());
            banner_input.set(user_profile.banner.get_clone());
            bio_input.set(user_profile.bio.get_clone());
            password_msg.set(String::new());
        }
    });

    let close = move |_| {
        modal_context.is_profile_settings_open.set(false);
    };

    let handle_save = move |_| {
        let new_name = name_input.get_clone();
        if !new_name.trim().is_empty() {
            user_profile.username.set(new_name);
        }
        user_profile.avatar.set(avatar_input.get_clone());
        user_profile.banner.set(banner_input.get_clone());
        user_profile.bio.set(bio_input.get_clone());
        modal_context.is_profile_settings_open.set(false);
    };

    let handle_change_password = move |_| {
        let n_pass = new_password.get_clone();
        let c_pass = confirm_password.get_clone();
        if n_pass.len() < 6 {
            password_msg.set("Пароль должен быть не менее 6 символов".to_string());
            return;
        }
        if n_pass != c_pass {
            password_msg.set("Пароли не совпадают".to_string());
            return;
        }
        password_msg.set("Пароль успешно обновлён!".to_string());
        old_password.set(String::new());
        new_password.set(String::new());
        confirm_password.set(String::new());
    };

    let handle_logout = move |_| {
        modal_context.is_profile_settings_open.set(false);
        screen_wrapper.set(ActiveScreen::Login);
    };

    view! {
        div(
            class="server-modal-overlay large",
            on:click=close,
        ) {
            div(
                class="server-modal profile-settings-modal",
                on:click=move |e: web_sys::MouseEvent| e.stop_propagation(),
            ) {
                div(class="profile-modal-top-bar") {
                    span(class="profile-modal-title") { "Настройки профиля" }
                    button(class="modal-close-icon-btn", on:click=close, title="Закрыть") { "✕" }
                }
                div(class="profile-modal-layout") {
                    div(class="profile-sidebar-tabs") {
                        h4 { "НАСТРОЙКИ" }
                        div(
                            class=classes(vec![
                                "profile-tab-item".into(),
                                ("active", { active_tab.get() == SettingsTab::Profile }.into()).into(),
                            ]),
                            on:click=move |_| active_tab.set(SettingsTab::Profile),
                        ) {
                            span { "👤 Профиль" }
                        }
                        div(
                            class=classes(vec![
                                "profile-tab-item".into(),
                                ("active", { active_tab.get() == SettingsTab::Security }.into()).into(),
                            ]),
                            on:click=move |_| active_tab.set(SettingsTab::Security),
                        ) {
                            span { "🔒 Безопасность" }
                        }

                        div(class="profile-tab-divider")

                        div(class="profile-tab-item logout", on:click=handle_logout) {
                            span { "🚪 Выйти из аккаунта" }
                        }
                    }

                    div(class="profile-content-body") {
                        (if active_tab.get() == SettingsTab::Profile {
                            view! {
                                div(class="tab-pane") {
                                    h3(class="tab-title") { "Мой профиль" }

                                    // Live preview card
                                    div(class="profile-card-preview") {
                                        div(class="profile-preview-banner") {
                                            (if !banner_input.get_clone().is_empty() {
                                                let b_url = banner_input.get_clone();
                                                view! { img(src=b_url, alt="") }
                                            } else {
                                                view! { div(class="default-banner") }
                                            })
                                        }

                                        div(class="profile-preview-header") {
                                            div(class="profile-preview-avatar") {
                                                (if !avatar_input.get_clone().is_empty() {
                                                    let a_url = avatar_input.get_clone();
                                                    view! { img(src=a_url, alt="") }
                                                } else {
                                                    let initial = name_input.get_clone().chars().next().unwrap_or('?').to_uppercase().to_string();
                                                    view! { span { (initial) } }
                                                })
                                                div(class="status-dot online")
                                            }
                                            div(class="profile-preview-names") {
                                                span(class="preview-name") { (name_input.get_clone()) }
                                                span(class="preview-tag") { "@" (name_input.get_clone().to_lowercase()) }
                                            }
                                        }

                                        (if !bio_input.get_clone().is_empty() {
                                            let bio = bio_input.get_clone();
                                            view! {
                                                div(class="profile-preview-bio") {
                                                    span(class="bio-label") { "О СЕБЕ" }
                                                    p { (bio) }
                                                }
                                            }
                                        } else {
                                            view! {}
                                        })
                                    }

                                    div(class="input-group") {
                                        label { "ОТОБРАЖАЕМОЕ ИМЯ" }
                                        input(r#type="text", placeholder="Ваше имя", bind:value=name_input)
                                    }

                                    div(class="input-group") {
                                        label { "URL АВАТАРКИ" }
                                        input(r#type="text", placeholder="https://example.com/avatar.png", bind:value=avatar_input)
                                    }

                                    div(class="input-group") {
                                        label { "URL БАННЕРА" }
                                        input(r#type="text", placeholder="https://example.com/banner.png", bind:value=banner_input)
                                    }

                                    div(class="input-group") {
                                        label { "О СЕБЕ" }
                                        textarea(rows="2", placeholder="Расскажите немного о себе...", bind:value=bio_input)
                                    }

                                    div(class="create-form-actions modal-footer-buttons") {
                                        button(class="back-btn", on:click=close) { "Отмена" }
                                        button(class="submit-btn create-submit", on:click=handle_save) { "Сохранить изменения" }
                                    }
                                }
                            }
                        } else {
                            view! {
                                div(class="tab-pane") {
                                    h3(class="tab-title") { "Безопасность и вход" }

                                    div(class="input-group") {
                                        label { "ТЕКУЩИЙ ПАРОЛЬ" }
                                        input(r#type="password", placeholder="••••••••", bind:value=old_password)
                                    }

                                    div(class="input-group") {
                                        label { "НОВЫЙ ПАРОЛЬ" }
                                        input(r#type="password", placeholder="Минимум 6 символов", bind:value=new_password)
                                    }

                                    div(class="input-group") {
                                        label { "ПОДТВЕРДИТЕ НОВЫЙ ПАРОЛЬ" }
                                        input(r#type="password", placeholder="••••••••", bind:value=confirm_password)
                                    }

                                    (if !password_msg.get_clone().is_empty() {
                                        let msg = password_msg.get_clone();
                                        view! {
                                            div(class="password-status-msg") { (msg) }
                                        }
                                    } else {
                                        view! {}
                                    })

                                    div(class="create-form-actions modal-footer-buttons") {
                                        button(class="back-btn", on:click=close) { "Отмена" }
                                        button(class="submit-btn", on:click=handle_change_password) { "Обновить пароль" }
                                    }

                                    div(class="security-divider")

                                    div(class="danger-zone") {
                                        h4 { "СЕССИЯ" }
                                        p { "Выход из текущей учётной записи на этом устройстве." }
                                        button(class="danger-btn logout-btn", on:click=handle_logout) { "🚪 Выйти из аккаунта" }
                                    }
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}
