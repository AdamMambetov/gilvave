use gilvave_core::dto::channel::ChannelType;
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::common::{ChannelContext, ServerContext, UiModalContext},
    gateway::service::WsService,
    http::api::Api,
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

    let current_server_id = create_memo(move || {
        server_context
            .current
            .with(|s| s.as_ref().map(|srv| srv.id))
    });

    create_effect(move || {
        let server_id_opt = current_server_id.get();

        if let Some(channel_id) = context.current.with_untracked(|c| c.as_ref().map(|ch| ch.id)) {
            context.current.set(None);
            spawn_local_scoped(async move {
                let _ = WsService::left_channel(channel_id).await;
            });
        }

        if let Some(server_id) = server_id_opt {
            context.text.set(vec![]);
            context.voice.set(vec![]);
            spawn_local_scoped(async move {
                if let Ok(channels) = Api::get_server_channels(server_id).await {
                    let mut text_channels = Vec::new();
                    let mut voice_channels = Vec::new();
                    for channel in channels {
                        match channel.r#type {
                            ChannelType::TEXT => text_channels.push(channel),
                            ChannelType::VOICE => voice_channels.push(channel),
                        }
                    }
                    context.text.set(text_channels);
                    context.voice.set(voice_channels);
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
                        ChannelItem(channel_view=channel)
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
