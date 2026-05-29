use gilvave_core::dto::{
    channel::{
        ChannelType::{TEXT, VOICE},
        ChannelView,
    },
    server::{MemberView, ServerView},
};
use serde::Serialize;
use sycamore::{futures::spawn_local_scoped, prelude::*};
use wasm_bindgen::JsValue;

use crate::{
    components::{
        common::{ActiveScreen, ScreenWrapper, classes},
        features::member_item::MemberItem,
    },
    utils::invoke,
};

#[derive(Clone)]
struct MemberContext(Signal<Vec<MemberView>>);

#[derive(Clone)]
struct ChannelContext {
    text: Signal<Vec<ChannelView>>,
    voice: Signal<Vec<ChannelView>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")] // Needed for tauri command
struct ServerIdArgs {
    server_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")] // Needed for tauri command
pub struct CreateServerArgs {
    pub name: String,
    pub icon_url: Option<String>,
    pub is_public: bool,
}

#[component]
pub fn HomePanel() -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();
    let is_home_screen: MaybeDyn<bool> = (move || screen_wrapper.is_home()).into();
    let server_list = create_signal::<Vec<ServerView>>(vec![]);
    let member_list = create_signal::<Vec<MemberView>>(vec![]);
    let text_channel_list = create_signal::<Vec<ChannelView>>(vec![]);
    let voice_channel_list = create_signal::<Vec<ChannelView>>(vec![]);

    let member_context = MemberContext(member_list);
    provide_context(member_context);
    let channel_context = ChannelContext {
        text: text_channel_list,
        voice: voice_channel_list,
    };
    provide_context(channel_context);

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

    let on_create_server = move || {
        spawn_local_scoped(async move {
            let args = serde_wasm_bindgen::to_value(&CreateServerArgs {
                name: "22".to_string(),
                icon_url: None,
                is_public: true,
            })
            .unwrap();
            let res = invoke("create_server", args).await;
            if let Ok(server_view) = serde_wasm_bindgen::from_value::<ServerView>(res) {
                server_list.update(|list| list.push(server_view));
            }
        });
    };

    view! {
        div(
            class=classes(vec![
                "discord-container".into(),
                "home-panel-container".into(),
                ("active", is_home_screen.clone()).into(),
            ]),
        ) {
            div(class="discord-sidebar") {
                div(class="server-icon", on:click=move |_| {
                    screen_wrapper.set(ActiveScreen::Login);
                    screen_wrapper.set(ActiveScreen::Home);
                }) { "🏠" }
                div(class="separator") {}
                Indexed(
                    list=server_list,
                    view=|server| {
                        let server_name = server.name;
                        view! {
                            div(
                                class="server-icon",
                                on:click=move |_| on_click_server(server.id.0.to_string()),
                            ) { (server_name) }
                        }
                    },
                )
                div(
                    class="server-icon new",
                    on:click=move |_| on_create_server(),
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
                    div(class="channel-panel") {
                        div(class="channel-list") {
                            div(class="channel-header") { "Текстовые" }

                            Indexed(
                                list=text_channel_list,
                                view=|channel| { view! { div(class="channel-item") { (channel.name) } } },
                            )
                            // div(class="channel-item active") { "Стандартный" }
                            // div(class="channel-item active") { "Второй" }
                        }

                        div(class="channel-list") {
                            div(class="channel-header") { "Голосовые" }

                            Indexed(
                                list=voice_channel_list,
                                view=|channel| { view! { div(class="channel-item") { (channel.name) } } },
                            )
                            // div(class="channel-item") { "Стандартный" }
                            // div(class="channel-item") { "Голос 2" }
                        }
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
                        view=|member| { view! { MemberItem(member_view=member) } },
                    )
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
        let args = serde_wasm_bindgen::to_value(&ServerIdArgs {
            server_id: server_id.clone(),
        })
        .unwrap();
        let res = invoke("get_members", args).await;
        if let Ok(members) = serde_wasm_bindgen::from_value::<Vec<MemberView>>(res) {
            context.0.set(members);
        } else {
            context.0.set(vec![]);
        }

        let context = use_context::<ChannelContext>();
        let args = serde_wasm_bindgen::to_value(&ServerIdArgs {
            server_id: server_id.clone(),
        })
        .unwrap();
        let res = invoke("get_server_channels", args).await;
        context.text.set(vec![]);
        context.voice.set(vec![]);

        if let Ok(channels) = serde_wasm_bindgen::from_value::<Vec<ChannelView>>(res) {
            for channel in channels {
                match channel.r#type {
                    TEXT => context.text.update(move |list| list.push(channel)),
                    VOICE => context.voice.update(move |list| list.push(channel)),
                }
            }
        }
    });
}
