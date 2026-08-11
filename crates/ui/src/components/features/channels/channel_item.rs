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
    let is_active = create_memo(move || context.current.get() == Some(channel_view.id));

    view! {
        div(
            class=classes(vec![
                "channel-item".into(),
                ("active", is_active.into()).into(),
            ]),
            on:click=move |_| {
                spawn_local_scoped(async move {
                    let context = use_context::<ChannelContext>();
                    if context.current.get().is_some() {
                        let args = CommandArgs::LeftChannel {
                            channel_id: context.current.get().unwrap()
                        }.to_json();
                        context.current.set(None);
                        invoke_command(args).await;
                    }

                    let args = CommandArgs::JoinChannel{
                        channel_id: channel_view.id
                    }.to_json();
                    let res = invoke_command(args).await;
                    if let CommandResult::Ok(CommandResponse::JoinChannel) = res {
                        context.current.set(Some(channel_view.id));
                    } else if let CommandResult::Error(err) = res {
                        console_error!("join channel error: {err:#?}");
                    }

                    let args = CommandArgs::ChannelHistoryBefore {
                        channel_id: channel_view.id,
                        timestamp: time::OffsetDateTime::now_utc(),
                    }.to_json();
                    invoke_command(args).await;
                });
            },
        ) {
            (channel_view.name)
        }
    }
}
