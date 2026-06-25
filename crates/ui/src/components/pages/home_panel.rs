use gilvave_core::{
    dto::{
        channel::ChannelView,
        command::{CommandArgs, CommandResponse, CommandResult},
        message::MessageView,
        server::{MemberView, ServerCreateInfo, ServerView},
    },
    ids::ChannelId,
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::{
        common::{ChannelContext, MemberContext, ScreenWrapper, classes},
        features::{
            channels::channel_panel::ChannelPanel, members::members_panel::MembersPanel,
            chat::messages_area::MessagesArea, servers::server_sidebar::ServerSidebar,
        },
    },
    utils::invoke_command,
};

#[component]
pub fn HomePanel() -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();
    let is_home_screen: MaybeDyn<bool> = (move || screen_wrapper.is_home()).into();
    let server_list = create_signal::<Vec<ServerView>>(vec![]);
    let member_list = create_signal::<Vec<MemberView>>(vec![]);
    let text_channel_list = create_signal::<Vec<ChannelView>>(vec![]);
    let voice_channel_list = create_signal::<Vec<ChannelView>>(vec![]);
    let message_list = create_signal::<Vec<MessageView>>(vec![]);
    let message_text = create_signal(String::new());
    let current_channel_id = create_signal::<Option<ChannelId>>(None);

    let member_context = MemberContext(member_list);
    provide_context(member_context);
    let channel_context = ChannelContext {
        text: text_channel_list,
        voice: voice_channel_list,
        current: current_channel_id,
    };
    provide_context(channel_context);

    create_effect(move || {
        if screen_wrapper.is_home() {
            spawn_local_scoped(async move {
                invoke_command(CommandArgs::ListenWebSocket.to_json()).await;
            });
            spawn_local_scoped(async move {
                let res = invoke_command(CommandArgs::GetUserServers.to_json()).await;
                server_list.set(
                    if let CommandResult::Ok(CommandResponse::GetUserServers(servers)) = res {
                        servers
                    } else {
                        vec![]
                    },
                );
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

    view! {
        div(
            class=classes(vec![
                "discord-container".into(),
                "home-panel-container".into(),
                ("active", is_home_screen.clone()).into(),
            ]),
        ) {
            ServerSidebar(
                server_list=server_list,
                on_create_server=on_create_server,
            )

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

                    MessagesArea(
                        message_list=message_list,
                        message_text=message_text,
                        current_channel_id=current_channel_id,
                    )
                }
            }

            MembersPanel()
        }
    }
}
