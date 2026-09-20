use sycamore::prelude::*;

use crate::{
    components::{
        common::{CreateServerContext, HomeTab, ModalView, UiModalContext, UserProfileContext},
        features::servers::open_join_modal,
    },
    utils::to_local_datetime,
};

#[component]
pub fn HomeDashboard() -> View {
    let user_profile = use_context::<UserProfileContext>();
    let modal_context = use_context::<UiModalContext>();
    let server_modal_context = use_context::<CreateServerContext>();

    let dm_input = create_signal(String::new());

    view! {
        (if let Some(dm_name) = modal_context.selected_dm_name.get_clone() {
            dm_chat_view(dm_name, modal_context.clone(), dm_input)
        } else {
            dashboard_main_view(
                user_profile.clone(),
                modal_context.clone(),
                server_modal_context.clone(),
            )
        })
    }
}

fn dashboard_main_view(
    user_profile: UserProfileContext,
    modal_context: UiModalContext,
    server_modal_context: CreateServerContext,
) -> View {
    let open_join = {
        let smc = server_modal_context.clone();
        move |_| {
            smc.from_dashboard.set(true);
            open_join_modal(smc.clone());
        }
    };

    let open_create = {
        let smc = server_modal_context.clone();
        move |_| {
            smc.modal_view.set(ModalView::Create);
            smc.is_modal_open.set(true);
        }
    };

    let open_settings = {
        let mc = modal_context.clone();
        move |_| {
            mc.is_profile_settings_open.set(true);
        }
    };

    let select_dm_smirnov = {
        let mc = modal_context.clone();
        move |_| {
            mc.selected_dm_name.set(Some("Алексей Смирнов".to_string()));
        }
    };

    let select_dm_vasilieva = {
        let mc = modal_context.clone();
        move |_| {
            mc.selected_dm_name.set(Some("Елена Васильева".to_string()));
        }
    };

    let go_back_to_chats = {
        let mc = modal_context.clone();
        move |_| {
            mc.home_tab.set(HomeTab::Chats);
        }
    };

    let greeting_text = create_memo(move || {
        let u_name = user_profile.username.get_clone();
        if u_name.is_empty() {
            "Добро пожаловать в Gilvave! 👋".to_string()
        } else {
            format!("Добро пожаловать, {}! 👋", u_name)
        }
    });

    view! {
        div(class="home-dashboard") {
            div(class="home-dashboard-mobile-header") {
                button(class="dashboard-back-btn", on:click=go_back_to_chats.clone()) {
                    "← К списку чатов"
                }
            }

            div(class="home-hero-banner") {
                div(class="hero-content") {
                    h1 { (greeting_text.get_clone()) }
                    p { "Ваше уютное пространство для текстовых, голосовых и видеочатов без ограничений." }
                }
                div(class="hero-decoration") {
                    span { "✨" }
                }
            }

            div(class="dashboard-section-title") {
                span { "БЫСТРЫЙ ДОСТУП" }
            }

            div(class="quick-actions-grid") {
                div(class="quick-card join", on:click=open_join) {
                    div(class="quick-card-icon") { "🌐" }
                    div(class="quick-card-text") {
                        span(class="quick-card-title") { "Каталог серверов" }
                        span(class="quick-card-desc") { "Найдите интересные публичные сообщества" }
                    }
                    span(class="quick-card-arrow") { "→" }
                }

                div(class="quick-card create", on:click=open_create) {
                    div(class="quick-card-icon") { "➕" }
                    div(class="quick-card-text") {
                        span(class="quick-card-title") { "Создать сервер" }
                        span(class="quick-card-desc") { "Создайте место для друзей или команды" }
                    }
                    span(class="quick-card-arrow") { "→" }
                }

                div(class="quick-card settings", on:click=open_settings) {
                    div(class="quick-card-icon") { "⚙️" }
                    div(class="quick-card-text") {
                        span(class="quick-card-title") { "Настройки профиля" }
                        span(class="quick-card-desc") { "Смените аватарку, никнейм и пароль" }
                    }
                    span(class="quick-card-arrow") { "→" }
                }

                div(class="quick-card dms", on:click=go_back_to_chats) {
                    div(class="quick-card-icon") { "💬" }
                    div(class="quick-card-text") {
                        span(class="quick-card-title") { "Личные чаты" }
                        span(class="quick-card-desc") { "Общайтесь один на один в безопасности" }
                    }
                    span(class="quick-card-arrow") { "→" }
                }
            }

            div(class="dashboard-section-title") {
                span { "ДРУЗЬЯ ОНЛАЙН" }
            }

            div(class="friends-online-list") {
                div(class="friend-card") {
                    div(class="friend-card-avatar") {
                        img(src="https://images.unsplash.com/photo-1534528741775-53994a69daeb?w=100", alt="")
                        div(class="status-dot online")
                    }
                    div(class="friend-card-info") {
                        span(class="friend-name") { "Алексей Смирнов" }
                        span(class="friend-status") { "В сети • Кодит на Rust 🦀" }
                    }
                    button(class="friend-action-btn", on:click=select_dm_smirnov) {
                        "Написать 💬"
                    }
                }

                div(class="friend-card") {
                    div(class="friend-card-avatar") {
                        img(src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100", alt="")
                        div(class="status-dot online")
                    }
                    div(class="friend-card-info") {
                        span(class="friend-name") { "Елена Васильева" }
                        span(class="friend-status") { "В сети • Разрабатывает UI" }
                    }
                    button(class="friend-action-btn", on:click=select_dm_vasilieva) {
                        "Написать 💬"
                    }
                }
            }
        }
    }
}

fn dm_chat_view(
    dm_name: String,
    modal_context: UiModalContext,
    dm_input: Signal<String>,
) -> View {
    let dm_title = dm_name.clone();
    let dm_initial = dm_name
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();
    let dm_start_text = format!("Это начало вашей истории личных сообщений с {}", dm_name);

    let today_date_text = {
        let now = to_local_datetime(time::OffsetDateTime::now_utc());
        let month_str = match now.month() {
            time::Month::January => "января",
            time::Month::February => "февраля",
            time::Month::March => "марта",
            time::Month::April => "апреля",
            time::Month::May => "мая",
            time::Month::June => "июня",
            time::Month::July => "июля",
            time::Month::August => "августа",
            time::Month::September => "сентября",
            time::Month::October => "октября",
            time::Month::November => "ноября",
            time::Month::December => "декабря",
        };
        format!("Сегодня, {} {}", now.day(), month_str)
    };

    let on_dm_send = move |e: web_sys::SubmitEvent| {
        e.prevent_default();
        let text = dm_input.get_clone();
        if !text.trim().is_empty() {
            dm_input.set(String::new());
        }
    };

    let hero_avatar = {
        let initial = dm_initial.clone();
        view! {
            div(class="dm-hero-avatar") {
                span { (initial) }
            }
        }
    };

    let hero_title = {
        let title = dm_title.clone();
        view! {
            h3 { (title) }
        }
    };

    let hero_text = view! {
        p { (dm_start_text) }
    };

    let header_avatar = {
        let initial = dm_initial.clone();
        view! {
            div(class="dm-header-avatar") {
                span { (initial) }
                div(class="status-dot online")
            }
        }
    };

    let header_name = {
        let title = dm_title.clone();
        view! {
            span(class="dm-header-name") { (title) }
        }
    };

    view! {
        div(class="messages-area dm-messages-area") {
            div(class="dm-chat-header") {
                button(
                    class="mobile-back-btn",
                    on:click=move |_| modal_context.selected_dm_name.set(None),
                ) { "← Назад" }
                (header_avatar)
                div(class="dm-header-meta") {
                    (header_name)
                    span(class="dm-header-sub") { "В сети • Личные сообщения" }
                }
            }

            div(class="messages-scroll") {
                div(class="dm-start-hero") {
                    (hero_avatar)
                    (hero_title)
                    (hero_text)
                }

                div(class="chat-date-divider") {
                    span(class="date-badge") { (today_date_text) }
                }

                div(class="message group-first group-last") {
                    div(class="message-avatar") {
                        span { "G" }
                    }
                    div(class="message-body") {
                        div(class="message-header") {
                            span(class="author") { "Gilvave Bot" }
                            span(class="timestamp") { "Сегодня, 12:00" }
                        }
                        div(class="message-bubble") {
                            p { "Привет! Это безопасный сквозной канал для общения. Напиши сообщение ниже!" }
                        }
                    }
                }
            }

            form(class="input-area", on:submit=on_dm_send) {
                input(
                    class="chat-input",
                    r#type="text",
                    placeholder="Напишите сообщение...",
                    bind:value=dm_input,
                )
                button(class="send-btn", r#type="submit", title="Отправить") {
                    svg(
                        xmlns="http://www.w3.org/2000/svg",
                        width="20",
                        height="20",
                        viewBox="0 0 24 24",
                        fill="none",
                        stroke="currentColor",
                        stroke-width="2",
                        stroke-linecap="round",
                        stroke-linejoin="round",
                    ) {
                        path(d="m22 2-7 20-4-9-9-4Z")
                        path(d="M22 2 11 13")
                    }
                }
            }
        }
    }
}
