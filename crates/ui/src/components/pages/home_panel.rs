use gilvave_core::dto::server::{MemberView, ServerView};
use serde::Serialize;
use sycamore::{futures::spawn_local_scoped, prelude::*};
use wasm_bindgen::JsValue;

use crate::{
    components::common::{ScreenWrapper, classes},
    utils::invoke,
};

#[derive(Clone)]
struct MemberContext(Signal<Vec<MemberView>>);

#[derive(Serialize)]
struct GetMembersArgs {
    serverId: String,
}

#[component]
pub fn HomePanel() -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();
    let is_home_screen: MaybeDyn<bool> = (move || screen_wrapper.is_home()).into();
    let server_list = create_signal::<Vec<ServerView>>(vec![]);
    let member_list = create_signal::<Vec<MemberView>>(vec![]);

    let member_context = MemberContext(member_list);
    provide_context(member_context);

    create_effect(move || {
        if screen_wrapper.is_home() {
            spawn_local_scoped(async move {
                let res = invoke("get_user_servers", JsValue::NULL).await;
                if let Ok(servers) = serde_wasm_bindgen::from_value::<Vec<ServerView>>(res) {
                    server_list.set(servers);
                } else {
                    server_list.set(vec![]);
                }
            });
        }
    });

    view! {
        div(
            class=classes(vec![
                "discord-container".into(),
                "home-panel-container".into(),
                ("active", is_home_screen.clone()).into(),
            ]),
        ) {
            div(class="discord-sidebar") {
                div(class="server-icon") { "🏠" }
                div(class="separator") {}
                Indexed(
                    list=server_list,
                    view=|server| {
                        let server_name = server.name;
                        view! {
                            div(class="server-icon", on:click=move |_| on_click_server(server.id.0.to_string())) { (server_name) }
                        }
                    },
                )
                div(
                    class="server-icon new",
                    //on:click=move |_| server_list.update(|list| list.push()),
                ) { "+" }
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

                    Indexed(
                        list=member_list,
                        view=|member| {
                            let member_name = format!("👤 {}", member.username);
                            view! {
                                div(class="member-item") { (member_name) }
                            }
                        },
                    )

                    // div(class="member-item") { "👤 Админ (online)" }
                    // div(class="member-item") { "👤 Модератор (online)" }
                    // div(class="member-item") { "👤 User (online)" }
                    // div(class="member-item") { "👤 Бот (online)" }
                }

                div(class="panel-section") {
                    h3 { "Офлайн" }
                    // div(class="member-item") { "👤 Guest (offline)" }
                    // div(class="member-item") { "👤 Stranger (offline)" }
                }
            }
        }
    }
}

fn on_click_server(server_id: String) {
    spawn_local_scoped(async move {
        let context = use_context::<MemberContext>();
        let args = serde_wasm_bindgen::to_value(&GetMembersArgs {
            serverId: server_id,
        })
        .unwrap();
        let res = invoke("get_members", args).await;
        if let Ok(members) = serde_wasm_bindgen::from_value::<Vec<MemberView>>(res) {
            context.0.set(members);
        } else {
            context.0.set(vec![]);
        }
    });
}
