use gilvave_core::dto::{
    channel::ChannelView,
    command::{CommandArgs, CommandResponse, CommandResult},
};
use sycamore::{futures::spawn_local_scoped, prelude::*, web::console_error};

use crate::utils::invoke_command;

#[component(inline_props)]
pub fn ChannelItem(channel_view: ChannelView, on_join_channel: fn() -> ()) -> View {
    view! {
        div(
            class="channel-item",
            on:click=move |_| {
                console_log!("click on channel 1");
                spawn_local_scoped(async move {
                    console_log!("click on channel 2");
                    let args = CommandArgs::JoinChannel{
                        channel_id: channel_view.id
                    }
                    .to_json();
                    console_log!("click on channel 3");
                    let res = invoke_command(args).await;
                    console_log!("click on channel 4");
                    if let CommandResult::Ok(CommandResponse::JoinChannel) = res {
                        console_log!("click on channel 5");
                        on_join_channel();
                        console_log!("click on channel 6");
                    } else if let CommandResult::Error(err) = res {
                        console_log!("click on channel 7");
                        console_error!("join channel error: {err:#?}");
                        console_log!("click on channel 8");
                    }
                    console_log!("click on channel 9");
                });
            },
        ) {
            (channel_view.name)
        }
    }
}
