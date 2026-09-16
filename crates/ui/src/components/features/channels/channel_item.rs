use gilvave_core::dto::{
    channel::ChannelView,
    command::{CommandArgs, CommandResponse, CommandResult},
};
use sycamore::{futures::spawn_local_scoped, prelude::*, web::console_error};

use crate::{
    components::common::{ChannelContext, classes},
    utils::invoke_command,
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

                let args = CommandArgs::LeftChannel {
                    channel_id: channel.id,
                }
                .to_json();
                context.current.set(None);
                context.messages.set(vec![]);
                invoke_command(args).await;
            }

            let args = CommandArgs::JoinChannel {
                channel_id: channel_item_id,
            }
            .to_json();
            let res = invoke_command(args).await;
            if let CommandResult::Ok(CommandResponse::JoinChannel) = res {
                context.current.set(Some(channel_item));
            } else if let CommandResult::Error(err) = res {
                console_error!("join channel error: {err:#?}");
            }

            let args = CommandArgs::ChannelHistoryBefore {
                channel_id: channel_item_id,
                timestamp: time::OffsetDateTime::now_utc(),
            }
            .to_json();
            invoke_command(args).await;
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
