use gilvave_core::dto::{
    channel::ChannelType,
    command::{CommandArgs, CommandResponse, CommandResult},
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::common::{ChannelContext, ServerContext, UiModalContext},
    utils::invoke_command,
};

use super::{channel_item::ChannelItem, user_status_bar::UserStatusBar};

#[component(inline_props)]
pub fn ChannelPanel() -> View {
    let context = use_context::<ChannelContext>();
    let server_context = use_context::<ServerContext>();
    let modal_context = use_context::<UiModalContext>();

    let on_add_text_channel = move |_| {
        modal_context.create_channel_type.set(ChannelType::TEXT);
        modal_context.is_create_channel_open.set(true);
    };

    let on_add_voice_channel = move |_| {
        modal_context.create_channel_type.set(ChannelType::VOICE);
        modal_context.is_create_channel_open.set(true);
    };

    create_effect(move || {
        spawn_local_scoped(async move {
            if let Some(channel) = context.current.get_clone() {
                let args = CommandArgs::LeftChannel {
                    channel_id: channel.id,
                }
                .to_json();
                context.current.set(None);
                invoke_command(args).await;
            }
        });

        if let Some(server) = server_context.current.get_clone() {
            spawn_local_scoped(async move {
                let args = CommandArgs::GetServerChannels {
                    server_id: server.id.clone(),
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
        } else {
            context.messages.set(vec![]);
            context.text.set(vec![]);
            context.voice.set(vec![]);
        }
    });

    view! {
        div(class="channel-panel") {
            div(class="channel-list") {
                div(class="channel-header") {
                    span { "Текстовые" }
                    button(
                        class="channel-add-btn",
                        on:click=on_add_text_channel,
                        title="Создать текстовый канал",
                    ) { "+" }
                }
                Indexed(
                    list=context.text,
                    view=|channel| { view! {
                        ChannelItem(channel_view=channel.clone())
                    }},
                )
            }

            div(class="channel-list") {
                div(class="channel-header") {
                    span { "Голосовые" }
                    button(
                        class="channel-add-btn",
                        on:click=on_add_voice_channel,
                        title="Создать голосовой канал",
                    ) { "+" }
                }
                Indexed(
                    list=context.voice,
                    view=|channel| { view! { div(class="channel-item") { (channel.name) } } },
                )
            }

            UserStatusBar()
        }
    }
}
