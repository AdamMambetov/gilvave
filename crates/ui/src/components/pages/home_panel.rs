use gilvave_core::dto::{
    channel::ChannelView,
    command::{CommandArgs, CommandResponse, CommandResult},
    message::MessageView,
    server::{MemberView, Server, ServerSmallPart},
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::{
        common::{ChannelContext, ScreenWrapper, ServerContext, classes},
        features::{
            channels::channel_panel::ChannelPanel, chat::messages_area::MessagesArea,
            members::members_panel::MembersPanel, servers::server_sidebar::ServerSidebar,
        },
        ui::icons::ServerIcon,
    },
    utils::invoke_command,
};

#[component]
pub fn HomePanel() -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();
    let is_home_screen: MaybeDyn<bool> = (move || screen_wrapper.is_home()).into();
    let user_name = create_signal(String::default());
    let user_icon = create_signal(String::default());

    spawn_local_scoped(async move {
        let args = CommandArgs::GetProfile.to_json();
        let res = invoke_command(args).await;
        if let CommandResult::Ok(CommandResponse::GetProfile(user)) = res {
            user_name.set(user.username);
            user_icon.set(user.avatar);
        }
    });

    let server_context = ServerContext {
        current: create_signal::<Option<Server>>(None),
        list: create_signal::<Vec<ServerSmallPart>>(vec![]),
        members: create_signal::<Vec<MemberView>>(vec![]),
    };
    provide_context(server_context.clone());

    let channel_context = ChannelContext {
        text: create_signal::<Vec<ChannelView>>(vec![]),
        voice: create_signal::<Vec<ChannelView>>(vec![]),
        current: create_signal::<Option<ChannelView>>(None),
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

    let handle_home_click = move |_| {
        spawn_local_scoped(async {
            let context = use_context::<ChannelContext>();
            if let Some(channel) = context.current.get_clone() {
                let args = CommandArgs::LeftChannel {
                    channel_id: channel.id,
                }
                .to_json();
                context.current.set(None);
                invoke_command(args).await;
            }
        });
        let context = use_context::<ServerContext>();
        context.current.set(None);
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
                ServerIcon(
                    server_name=user_name,
                    icon_url=user_icon,
                    on:click=handle_home_click,
                )
                div(class="separator")
                ServerSidebar()
            }

            div(class="discord-main") {
                div(class="discord-header") {
                    div(class="search-bar") {
                        span { "🔍 Поиск" }
                    }
                }

                div(class="discord-content") {
                    ChannelPanel()

                    (if channel_context.current.get_clone().is_some() {
                        MessagesArea()
                    } else {
                        view!{}
                    })
                }
            }

            (if server_context.current.get_clone().is_some() {
                MembersPanel()
            } else {
                view!{}
            })
        }
    }
}
