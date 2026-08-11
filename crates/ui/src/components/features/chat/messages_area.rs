use futures_util::StreamExt;
use gilvave_core::{
    dto::{command::CommandArgs, message::MessageView},
    ids::ChannelId,
};
use sycamore::{futures::spawn_local_scoped, prelude::*, web::queue_microtask};
use tauri_sys::event::listen;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, SubmitEvent};

use super::message_item::MessageItem;
use crate::{components::common::ChannelContext, utils::invoke_command};

#[component(inline_props)]
pub fn MessagesArea(
    message_list: Signal<Vec<MessageView>>,
    message_text: Signal<String>,
    current_channel_id: Signal<Option<ChannelId>>,
) -> View {
    let container = create_node_ref();
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let msg = message_text.get_clone().trim().to_string();
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
    let on_scroll = move |event: Event| {
        let el: HtmlElement = match event.current_target() {
            Some(t) => t.unchecked_into(),
            None => return,
        };

        const SCROLL_THRESHOLD: i32 = 1;

        let top = el.scroll_top();
        let sh = el.scroll_height();
        let ch = el.client_height();

        let is_top = top <= 0;
        let is_bottom = (sh - ch - top).abs() <= SCROLL_THRESHOLD;

        if is_top {
            console_log!("top message");
            spawn_local_scoped(async move {
                let context = use_context::<ChannelContext>();
                let args = CommandArgs::ChannelHistoryBefore {
                    channel_id: context.current.get().unwrap(),
                    timestamp: message_list.get_clone().first().unwrap().created_at,
                }
                .to_json();
                invoke_command(args).await;
            });
        }
        if is_bottom {
            console_log!("bottom message");
            spawn_local_scoped(async move {
                let context = use_context::<ChannelContext>();
                let args = CommandArgs::ChannelHistoryAfter {
                    channel_id: context.current.get().unwrap(),
                    timestamp: message_list.get_clone().last().unwrap().created_at,
                }
                .to_json();
                invoke_command(args).await;
            });
        }
    };

    spawn_local_scoped(async move {
        let mut events = listen::<MessageView>("message_new").await.unwrap();
        while let Some(event) = events.next().await {
            message_list.update(|list| list.push(event.payload));
        }
    });
    spawn_local_scoped(async move {
        let mut events = listen::<Vec<MessageView>>("channel_history_before")
            .await
            .unwrap();
        while let Some(event) = events.next().await {
            let el = container.get().unchecked_into::<HtmlElement>();
            let old_scroll_top = el.scroll_top();
            let old_scroll_height = el.scroll_height();

            message_list.update(|list| {
                for msg in event.payload.into_iter() {
                    list.insert(0, msg);
                }
            });

            // Ждём, пока Sycamore реально вольёт узлы в DOM.
            // queue_microtask достаточно, т.к. эффекты Sycamore выполняются
            // в микротасках. Для надёжности можно использовать RAF.
            queue_microtask(move || {
                // let el = container.get().unchecked_into::<HtmlElement>();
                let delta = el.scroll_height() - old_scroll_height;
                console_log!("old_scroll_top: {old_scroll_top}");
                console_log!("old_scroll_height: {old_scroll_height}");
                console_log!("delta: {delta}");
                let new_scroll_top = old_scroll_top + delta;
                console_log!("new_scroll_top: {new_scroll_top}");
                el.set_scroll_top(old_scroll_top + delta);
            });
        }
    });
    spawn_local_scoped(async move {
        let mut events = listen::<Vec<MessageView>>("channel_history_after")
            .await
            .unwrap();
        while let Some(mut event) = events.next().await {
            message_list.update(|list| list.append(event.payload.as_mut()));
        }
    });

    view! {
        div(class="messages-area") {
            div(class="messages-list", r#ref=container, on:scroll=on_scroll) {
                Keyed(
                    list=message_list,
                    key=|m| m.id,
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
