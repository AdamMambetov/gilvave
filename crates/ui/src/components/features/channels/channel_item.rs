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
    let context = use_context::<ChannelContext>();
    let channel_cc = Box::new(channel_view.clone());
    let is_active = create_memo(move || match context.current.get_clone() {
        Some(channel) => channel.id == channel_cc.id,
        None => false,
    });

    // TODO: Исправить миллион клонирований!
    let channel_ccc = channel_view.clone();
    let channel_cc = channel_view.clone();
    let on_click = move |_| {
        let channel_c = channel_ccc.clone();
        spawn_local_scoped(async move {
            let context = use_context::<ChannelContext>();
            if let Some(channel) = context.current.get_clone() {
                if channel.id == channel_c.id {
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
                channel_id: channel_c.id,
            }
            .to_json();
            let res = invoke_command(args).await;
            if let CommandResult::Ok(CommandResponse::JoinChannel) = res {
                // context.current.set(None);
                // context.current.set(Some(*channel_c));
            } else if let CommandResult::Error(err) = res {
                console_error!("join channel error: {err:#?}");
            }

            let args = CommandArgs::ChannelHistoryBefore {
                channel_id: channel_c.id,
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
            (channel_cc.name.clone())
        }
    }
}
