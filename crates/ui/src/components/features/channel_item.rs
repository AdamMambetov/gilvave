use gilvave_core::{
    dto::{
        channel::ChannelView,
        command::{CommandArgs, CommandResponse, CommandResult},
    },
    ids::ChannelId,
};
use sycamore::{futures::spawn_local_scoped, prelude::*, web::console_error};

use crate::utils::invoke_command;

#[component(inline_props)]
pub fn ChannelItem(channel_view: ChannelView, on_join_channel: fn(ChannelId) -> ()) -> View {
    view! {
        div(
            class="channel-item",
            on:click=move |_| {
                spawn_local_scoped(async move {
                    let args = CommandArgs::JoinChannel{
                        channel_id: channel_view.id
                    }
                    .to_json();
                    let res = invoke_command(args).await;
                    if let CommandResult::Ok(CommandResponse::JoinChannel) = res {
                        on_join_channel(channel_view.id);
                    } else if let CommandResult::Error(err) = res {
                        console_error!("join channel error: {err:#?}");
                    }
                });
            },
        ) {
            (channel_view.name)
        }
    }
}
