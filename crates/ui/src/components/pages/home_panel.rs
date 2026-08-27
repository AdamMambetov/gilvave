use gilvave_core::{
    dto::{
        channel::ChannelView,
        command::{CommandArgs, CommandResponse, CommandResult},
        message::MessageView,
        server::{MemberView, Server, ServerSmallPart},
    },
    ids::ChannelId,
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::{
        common::{ChannelContext, ScreenWrapper, ServerContext, classes},
        features::{
            channels::channel_panel::ChannelPanel, chat::messages_area::MessagesArea,
            members::members_panel::MembersPanel, servers::server_sidebar::ServerSidebar,
        },
    },
    utils::invoke_command,
};

#[component]
pub fn HomePanel() -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();
    let is_home_screen: MaybeDyn<bool> = (move || screen_wrapper.is_home()).into();

    let server_context = ServerContext {
        current: create_signal::<Option<Server>>(None),
        list: create_signal::<Vec<ServerSmallPart>>(vec![]),
        members: create_signal::<Vec<MemberView>>(vec![]),
    };
    provide_context(server_context.clone());

    let channel_context = ChannelContext {
        text: create_signal::<Vec<ChannelView>>(vec![]),
        voice: create_signal::<Vec<ChannelView>>(vec![]),
        current_id: create_signal::<Option<ChannelId>>(None),
        messages: create_signal::<Vec<MessageView>>(vec![]),
    };
    provide_context(channel_context.clone());

    create_effect(move || {
        if screen_wrapper.is_home() {
            spawn_local_scoped(async move {
                invoke_command(CommandArgs::ListenWebSocket.to_json()).await;
            });
            spawn_local_scoped(async move {
                let res = invoke_command(CommandArgs::GetUserServers.to_json()).await;
                server_context.list.set(
                    if let CommandResult::Ok(CommandResponse::GetUserServers(servers)) = res {
                        servers
                    } else {
                        vec![]
                    },
                );
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
            ServerSidebar()

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
                    ChannelPanel()
                    MessagesArea()
                }
            }

            MembersPanel()
        }
    }
}
