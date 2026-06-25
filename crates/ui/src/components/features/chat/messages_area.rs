use futures_util::StreamExt;
use gilvave_core::{
    dto::{
        command::CommandArgs,
        message::MessageView,
    },
    ids::ChannelId,
};
use sycamore::{futures::spawn_local_scoped, prelude::*};
use tauri_sys::event::listen;
use web_sys::SubmitEvent;

use super::message_item::MessageItem;
use crate::utils::invoke_command;

#[component(inline_props)]
pub fn MessagesArea(
    message_list: Signal<Vec<MessageView>>,
    message_text: Signal<String>,
    current_channel_id: Signal<Option<ChannelId>>,
) -> View {
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let msg = message_text.get_clone();
        let channel_id = current_channel_id.get();
        if msg.is_empty() || channel_id.is_none() {
            return;
        }

        spawn_local_scoped(async move {
            let args = CommandArgs::MessageCreate {
                channel_id: channel_id.unwrap(),
                content: msg,
            }
            .to_json();
            invoke_command(args).await;
        });
        message_text.set(String::new());
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
        div(class="messages-area") {
            div(class="messages-list") {
                Indexed(
                    list=message_list,
                    view=|message| { view! { MessageItem(message_view=message) } },
                )
            }

            form(class="input-area", on:submit=on_submit) {
                input(
                    class="chat-input",
                    placeholder="Напишите сообщение...",
                    bind:value=message_text,
                )
            }
        }
    }
}
