use gilvave_core::dto::channel::ChannelView;
use sycamore::{futures::spawn_local_scoped, prelude::*, web::console_error};

use crate::{
    components::common::{ChannelContext, classes},
    gateway::service::WsService,
};

#[component(inline_props)]
pub fn ChannelItem(channel_view: ChannelView) -> View {
    let channel_name = channel_view.name.clone();
    let channel_id = channel_view.id;
    let channel_signal = create_signal(channel_view);

    let context = use_context::<ChannelContext>();
    let is_active = create_memo(move || match context.current.get_clone() {
        Some(channel) => channel.id == channel_id,
        None => false,
    });

    let on_click = move |_| {
        spawn_local_scoped(async move {
            let channel_item = channel_signal.get_clone();
            let channel_item_id = channel_item.id;

            let context = use_context::<ChannelContext>();
            if let Some(channel) = context.current.get_clone() {
                if channel.id == channel_item_id {
                    return;
                }

                context.current.set(None);
                context.messages.set(vec![]);
                let _ = WsService::left_channel(channel.id).await;
            }

            web_sys::console::log_1(&format!("[CHANNEL_ITEM] clicked channel: {channel_item_id}").into());
            let res = WsService::join_channel(channel_item_id).await;
            web_sys::console::log_1(&format!("[CHANNEL_ITEM] JoinChannel res: {res:?}").into());
            match res {
                Ok(()) => {
                    context.current.set(Some(channel_item));
                    web_sys::console::log_1(&format!("[CHANNEL_ITEM] context.current updated to Some({channel_item_id})").into());
                }
                Err(err) => {
                    console_error!("join channel error: {err:#?}");
                }
            }

            let _ = WsService::channel_history_before(
                channel_item_id,
                time::OffsetDateTime::now_utc(),
            )
            .await;
        });
    };

    view! {
        div(
            class=classes(vec![
                "channel-item".into(),
                ("active", is_active.into()).into(),
            ]),
            on:click=on_click,
        ) {
            (channel_name)
        }
    }
}
