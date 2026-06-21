use futures_util::StreamExt;
use gilvave_core::{
    dto::{
        channel::{
            ChannelType::{self},
            ChannelView,
        },
        command::{CommandArgs, CommandResponse, CommandResult},
        message::MessageView,
        server::{MemberView, ServerCreateInfo, ServerView},
    },
    ids::ServerId,
};
use sycamore::{futures::spawn_local_scoped, prelude::*};
use tauri_sys::event::listen;

use crate::{
    components::{
        common::{ActiveScreen, ScreenWrapper, classes},
        features::{channel_item::ChannelItem, member_item::MemberItem, message_item::MessageItem},
    },
    utils::invoke_command,
};

#[derive(Clone)]
struct MemberContext(Signal<Vec<MemberView>>);

#[derive(Clone)]
struct ChannelContext {
    text: Signal<Vec<ChannelView>>,
    voice: Signal<Vec<ChannelView>>,
}

#[component]
pub fn HomePanel() -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();
    let is_home_screen: MaybeDyn<bool> = (move || screen_wrapper.is_home()).into();
    let server_list = create_signal::<Vec<ServerView>>(vec![]);
    let member_list = create_signal::<Vec<MemberView>>(vec![]);
    let text_channel_list = create_signal::<Vec<ChannelView>>(vec![]);
    let voice_channel_list = create_signal::<Vec<ChannelView>>(vec![]);
    let message_list = create_signal::<Vec<MessageView>>(vec![]);

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
                invoke_command(CommandArgs::ListenWebSocket.to_json()).await;
            });
            spawn_local_scoped(async move {
                let res = invoke_command(CommandArgs::GetUserServers.to_json()).await;
                if let CommandResult::Ok(CommandResponse::GetUserServers(servers)) = res {
                    server_list.set(servers);
                } else {
                    server_list.set(vec![]);
                }
            });
        }
    });

    let on_create_server = move || {
        spawn_local_scoped(async move {
            let args = CommandArgs::CreateServer {
                server_info: ServerCreateInfo {
                    name: "22".to_string(),
                    icon_url: None,
                    is_public: true,
                },
            }
            .to_json();
            let res = invoke_command(args).await;
            if let CommandResult::Ok(CommandResponse::CreateServer(server_view)) = res {
                server_list.update(|list| list.push(server_view));
            }
        });
    };

    spawn_local_scoped(async move {
        let mut events = listen::<MessageView>("message_new").await.unwrap();
        while let Some(event) = events.next().await {
            console_log!(
                "listen message_new '{}' from {}",
                event.payload.content,
                event.payload.author_name
            );
            message_list.update(|list| list.push(event.payload));
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
                                on:click=move |_| on_click_server(server.id),
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
                                view=|channel| { view! {
                                    ChannelItem(
                                        channel_view=channel,
                                        on_join_channel=|| {
                                            console_log!("join channel success from ui");
                                        },
                                    )
                                }},
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
                        Indexed(
                            list=message_list,
                            view=|message| { view! { MessageItem(message_view=message) } },
                        )

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

fn on_click_server(server_id: ServerId) {
    spawn_local_scoped(async move {
        let context = use_context::<MemberContext>();
        let args = CommandArgs::GetMembers {
            server_id: server_id.clone(),
        }
        .to_json();
        let res = invoke_command(args).await;

        context.0.set(vec![]);
        if let CommandResult::Ok(CommandResponse::GetMembers(members)) = res {
            context.0.set(members);
        }
    });
    spawn_local_scoped(async move {
        let context = use_context::<ChannelContext>();
        let args = CommandArgs::GetServerChannels {
            server_id: server_id.clone(),
        }
        .to_json();
        let res = invoke_command(args).await;

        context.text.set(vec![]);
        context.voice.set(vec![]);
        if let CommandResult::Ok(CommandResponse::GetServerChannels(channels)) = res {
            for channel in channels {
                match channel.r#type {
                    ChannelType::TEXT => context.text.update(|list| list.push(channel)),
                    ChannelType::VOICE => context.voice.update(|list| list.push(channel)),
                }
            }
        }
    })
}
