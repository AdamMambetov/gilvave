use gilvave_core::dto::{
    command::{CommandArgs, CommandResponse, CommandResult},
    server::ServerView,
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::common::{ActiveScreen, ChannelContext, MemberContext, ScreenWrapper},
    utils::invoke_command,
};

#[component(inline_props)]
pub fn ServerSidebar(
    server_list: Signal<Vec<ServerView>>,
    on_create_server: impl Fn() + 'static,
) -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();

    let handle_home_click = move |_| {
        screen_wrapper.set(ActiveScreen::Login);
        screen_wrapper.set(ActiveScreen::Home);
    };

    view! {
        div(class="discord-sidebar") {
            div(class="server-icon", on:click=handle_home_click) { "🏠" }
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
    }
}

fn on_click_server(server_id: gilvave_core::ids::ServerId) {
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
                    gilvave_core::dto::channel::ChannelType::TEXT => {
                        context.text.update(|list| list.push(channel))
                    }
                    gilvave_core::dto::channel::ChannelType::VOICE => {
                        context.voice.update(|list| list.push(channel))
                    }
                }
            }
        }
    })
}
