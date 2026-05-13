use sycamore::prelude::*;

use crate::components::common::classes;

#[derive(Props)]
pub struct HomePanelProps {
    visible: MaybeDyn<bool>,
}

#[component]
pub fn HomePanel(props: HomePanelProps) -> View {
    view! {
        div(
            class=classes(vec![
                "discord-container".into(),
                "home-panel-container".into(),
                ("active", props.visible.clone()).into()
            ]),
        ) {
            div(class="discord-sidebar") {
                div(class="server-icon") { "🏠" }
                div(class="server-icon") { "💬" }
                div(class="server-icon") { "👥" }
                div(class="server-icon") { "🔔" }
                div(class="server-icon new") { "+" }
                div(class="separator") {}
                div(class="server-icon") { "🎮" }
                div(class="server-icon") { "🎵" }
                div(class="server-icon") { "🎬" }
            }

            div(class="discord-main") {
                div(class="discord-header") {
                    div(class="search-bar") {
                        span { "🔍 Поиск" }
                    }
                    div(class="user-info") {
                        span { "Добро пожаловать, " }
                        span(class="user-name") { "User" }
                    }
                }

                div(class="discord-content") {
                    div(class="channel-list") {
                        div(class="channel-header") { "Участники" }
                        div(class="channel-item active") { "Админы" }
                        div(class="channel-item") { "Модераторы" }
                        div(class="channel-item") { "Гости" }
                        div(class="channel-item") { "🤖 Боты" }
                    }

                    div(class="messages-area") {
                        div(class="message") {
                            div(class="message-header") {
                                span(class="author") { "System" }
                                span(class="timestamp") { "Сейчас" }
                            }
                            p { "Добро пожаловать в Discord-подобный интерфейс! 🎉" }
                        }

                        div(class="message") {
                            div(class="message-header") {
                                span(class="author") { "Admin" }
                                span(class="timestamp") { "Сейчас" }
                            }
                            p { "Привет всем! Как дела?" }
                        }

                        div(class="message") {
                            div(class="message-header") {
                                span(class="author") { "User" }
                                span(class="timestamp") { "Сейчас" }
                            }
                            p { "Привет! Все отлично! 👍" }
                        }

                        div(class="message") {
                            div(class="message-header") {
                                span(class="author") { "Admin" }
                                span(class="timestamp") { "Сейчас" }
                            }
                            p { "Отлично! Создайте чат и начните общение!" }
                        }

                        div(class="input-area") {
                            input(class="chat-input", placeholder="Напишите сообщение...")
                            button(class="chat-btn") { "✈️" }
                        }
                    }
                }
            }

            div(class="discord-members-panel") {
                div(class="panel-section") {
                    h3 { "Онлайн" }
                    div(class="member-item") { "👤 Админ (online)" }
                    div(class="member-item") { "👤 Модератор (online)" }
                    div(class="member-item") { "👤 User (online)" }
                    div(class="member-item") { "👤 Бот (online)" }
                }

                div(class="panel-section") {
                    h3 { "Офлайн" }
                    div(class="member-item") { "👤 Guest (offline)" }
                    div(class="member-item") { "👤 Stranger (offline)" }
                }
            }
        }
    }
}
